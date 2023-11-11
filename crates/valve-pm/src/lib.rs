#![feature(try_blocks)]

use btleplug::platform::Adapter;
use btleplug::{
	api::{Central, Manager as _, Peripheral, ScanFilter, WriteType},
	platform::Manager,
};
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::{result, str::FromStr, time::Duration};
use tokio::{select, sync::mpsc, time::sleep};
use tracing::{error, info, warn, Instrument};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("bluetooth: {0}")]
	Bt(#[from] btleplug::Error),
	#[error("no adapter found")]
	NoAdapterFound,
	#[error("characteristic not found")]
	CharacteristicNotFound,
}

type Result<T, E = Error> = result::Result<T, E>;

const MODE_CHARACTERISTIC_ID: Lazy<Uuid> =
	Lazy::new(|| Uuid::from_str("00001525-1212-efde-1523-785feabcd124").expect("uuid is valid"));

#[allow(dead_code)]
const HTC_MAC_PREFIX: [u8; 3] = [0x74, 0xf6, 0x1c];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum StationState {
	Sleeping = 0x00,
	On = 0x01,
	Standby = 0x02,
	Unknown = 0xff,
}
impl StationState {
	fn from_id(id: u8) -> Self {
		match id {
			0x00 => Self::Sleeping,
			0x01 | 0x08 | 0x09 | 0x0a | 0x0b => Self::On,
			0x02 => Self::Standby,
			_ => {
				warn!("unknown status: 0x{id:02x}");
				Self::Unknown
			}
		}
	}
}

pub enum StationCommand {
	SetState(StationState),
}

pub async fn start_manager() -> Result<Arc<(Manager, Adapter)>, ()> {
	let manager = match Manager::new().await {
		Ok(manager) => manager,
		Err(err) => {
			error!("failed to create station manager: {err}");
			return Err(());
		}
	};
	let adapters = match manager.adapters().await {
		Ok(adapters) => adapters,
		Err(err) => {
			error!("failed to enumerate adapters: {err}");
			return Err(());
		}
	};
	if adapters.is_empty() {
		error!("no available bluetooth adapters");
		return Err(());
	}

	let adapter = adapters
		.iter()
		// Prefer HTC adapter, which is already embedded in link box
		// .find(|a| {
		// 	let addr = if let Ok(addr) = a.name() {
		// 		addr
		// 	} else {
		// 		return false;
		// 	};
		// 	!addr.address.starts_with(&HTC_MAC_PREFIX)
		// })
		// TODO: Ability to select adapter?
		// .or_else(|| adapters.first())
		.next()
		.expect("len >= 1")
		.clone();

	info!(
		"using adapter: {}",
		adapter
			.adapter_info()
			.await
			.unwrap_or_else(|_| "<unknown>".to_owned())
	);

	let _ = adapter.stop_scan().await;
	if let Err(e) = adapter.start_scan(ScanFilter::default()).await {
		warn!("failed to start scan: {e}");
	}

	Ok(Arc::new((manager, adapter)))
}

pub struct StationControl {
	handle: Option<tokio::task::JoinHandle<()>>,
	ctx: Option<mpsc::UnboundedSender<StationCommand>>,
}
impl StationControl {
	#[tracing::instrument(name = "station_control")]
	pub fn new(
		man_adapter: Arc<(Manager, Adapter)>,
		station_sn: String,
		initial_state: StationState,
	) -> Self {
		let (ctx, mut crx) = mpsc::unbounded_channel();
		let handle = tokio::task::spawn(
			async move {

				'rescan: loop {
					let adapter = &man_adapter.1;
					let res: Result<()> = try {
						let peripherals = adapter.peripherals().await?;

						for peripheral in peripherals {
							let props = match peripheral.properties().await {
								Ok(props) => props,
								Err(err) => {
									warn!("failed to get peripheral properties: {err}");
									continue;
								}
							};
							let name = props.and_then(|p| p.local_name);
							if name.as_ref() == Some(&station_sn) {
								info!("station found");
							} else {
								continue;
							}
							if !peripheral.is_connected().await? {
								info!("connecting");
								peripheral.connect().await?;
							}

							// Against base station flakiness
							tokio::time::sleep(Duration::from_millis(1500)).await;

							if peripheral.characteristics().is_empty() {
								info!("discovering characteristics");
								peripheral.discover_services().await?;
							}

							let characteristics = peripheral
								.characteristics();
							let characteristic = characteristics
								.iter()
								.find(|c| c.uuid == *MODE_CHARACTERISTIC_ID)
								.ok_or(Error::CharacteristicNotFound)?;

							if let Err(e) = peripheral.write(&characteristic, &[initial_state as u8], WriteType::WithoutResponse).await {
								error!("failed to set initial state: {e}");
							}

							peripheral.subscribe(characteristic).await?;

							let mut notifications = peripheral.notifications().await?;
							info!("waiting for commands");
							loop {
								select! {
									notification = notifications.next() => {
										let Some(notification) = notification else {
											warn!("device was disconnected");
											continue 'rescan;
										};
										if notification.uuid == *MODE_CHARACTERISTIC_ID {
											let state = if notification.value.len() == 1 {
												StationState::from_id(notification.value[0])
											} else {
												StationState::Unknown
											};
											info!("station state changed to {:?}", state);
										}
									}
									command = crx.recv() => {
										let Some(command) = command else {
											warn!("command stream finished, moving device to sleep");
											if let Err(e) = peripheral.write(&characteristic, &[StationState::Sleeping as u8], WriteType::WithoutResponse).await {
												error!("failed to make device sleep: {e}");
											}
											if let Err(e) = peripheral.disconnect().await {
												error!("failed to disconnect: {e}");
											}
											break 'rescan;
										};
										match command {
											StationCommand::SetState(state) => {
												info!("changing station state to {state:?}");
												if let Err(e) = peripheral.write(&characteristic, &[state as u8], WriteType::WithoutResponse).await {
													error!("failed to write station state: {e}");
												}
											},
										}
									}
								}
							}
						}
					};
					if let Err(e) = res {
						warn!("communication failed: {e}");
					}
					sleep(Duration::from_secs(5)).await;
				}
			}
			.in_current_span(),
		);
		Self {
			handle: Some(handle),
			ctx: Some(ctx),
		}
	}
	pub fn send(&mut self, command: StationCommand) {
		self.ctx
			.as_mut()
			.expect("connection is not finished")
			.send(command)
			.ok()
			.expect("task can't end earlier")
	}
	pub async fn finish(mut self) {
		drop(self.ctx.take().expect("finish can only be called once"));

		self.handle
			.take()
			.expect("finish can only be called once")
			.await
			.expect("task should not fail");
	}
}
