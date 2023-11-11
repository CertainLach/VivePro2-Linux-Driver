use std::{os::raw::c_char, sync::Mutex};

use crate::{
	driver_context::{try_init_driver_context, DRIVER_CONTEXT},
	factory::{get_hmd_driver_factory, TOKIO_RUNTIME},
	log::try_init_driver_log,
	setting,
	settings::Setting,
	try_vr,
};
use cppvtbl::{impl_vtables, HasVtable, VtableRef, WithVtables};
use once_cell::sync::Lazy;
use openvr::{IVRDriverLogVtable, IVRDriverLog_Version};
use tokio::task::LocalSet;
use tracing::info;
use valve_pm::{start_manager, StationCommand, StationControl, StationState};

use crate::openvr::{
	EVRInitError, IServerTrackedDeviceProvider, IServerTrackedDeviceProviderVtable,
	IServerTrackedDeviceProvider_Version, IVRDriverContextVtable,
};

// (name ":" "BS2" ":" "0"/"1") ** ","
const BASE_STATIONS: Setting<String> = setting!("driver_lighthouse", "PowerManagedBaseStations2");
// 0 - disabled
// 1 - sleep
// 2 - standby
const POWER_MANAGEMENT: Setting<i32> = setting!("vivepro2", "basestationPowerManagement");

#[impl_vtables(IServerTrackedDeviceProvider)]
pub struct ServerTrackedProvider {
	real: &'static VtableRef<IServerTrackedDeviceProviderVtable>,
	stations: Mutex<Vec<StationControl>>,
	standby_state: Mutex<StationState>,
}
impl IServerTrackedDeviceProvider for ServerTrackedProvider {
	fn Init(
		&self,
		pDriverContext: *const cppvtbl::VtableRef<IVRDriverContextVtable>,
	) -> EVRInitError {
		try_init_driver_context(unsafe { &*pDriverContext });
		let context = DRIVER_CONTEXT.get().expect("context just initialized");
		let logger: *const cppvtbl::VtableRef<IVRDriverLogVtable> = context
			.get_generic_interface(IVRDriverLog_Version)
			.expect("always able to initialize driver log")
			.cast();
		try_init_driver_log(unsafe { &*logger });

		let power_management = POWER_MANAGEMENT.get();
		*self.standby_state.lock().expect("lock") = match power_management {
			0 => StationState::Unknown,
			2 => StationState::Standby,
			_ => StationState::Sleeping,
		};

		'stations: {
			if *self.standby_state.lock().expect("lock") != StationState::Unknown {
				let _runtime = TOKIO_RUNTIME.enter();
				let stations = BASE_STATIONS.get();

				let stations: Vec<_> = stations.split(",").filter(|s| !s.is_empty()).collect();
				if stations.is_empty() {
					break 'stations;
				}
				let Ok(manager) = TOKIO_RUNTIME.block_on(start_manager()) else {
					break 'stations;
				};
				let stations: Vec<_> = stations
					.iter()
					.filter_map(|line| {
						let mut parts = line.split(":");
						let name = parts.next()?;
						let _bs2 = parts.next()?;
						let enabled = parts.next()?;

						if enabled == "1" {
							Some(name.to_owned())
						} else {
							None
						}
					})
					.map(|name| {
						StationControl::new(manager.clone(), name.to_owned(), StationState::On)
					})
					.collect();
				info!("enabled power management for {} stations", stations.len());
				self.stations.lock().expect("lock").extend(stations);
			}
		};

		self.real.Init(
			VtableRef::into_raw(HasVtable::<IVRDriverContextVtable>::get(&context)) as *const _,
		)
	}

	fn Cleanup(&self) {
		self.real.Cleanup();
		info!("disconnecting from base stations");
		let _runtime = TOKIO_RUNTIME.enter();
		let localset = LocalSet::new();
		for station in self.stations.lock().expect("lock").drain(..) {
			localset.spawn_local(station.finish());
		}
		TOKIO_RUNTIME.block_on(localset);
	}

	fn GetInterfaceVersions(&self) -> *const *const c_char {
		self.real.GetInterfaceVersions()
	}

	fn RunFrame(&self) {
		self.real.RunFrame()
	}

	fn ShouldBlockStandbyMode(&self) -> bool {
		false
	}

	fn EnterStandby(&self) {
		self.real.EnterStandby();
		info!("making station standby");
		for station in self.stations.lock().expect("lock").iter_mut() {
			station.send(StationCommand::SetState(
				*self.standby_state.lock().expect("lock"),
			))
		}
	}

	fn LeaveStandby(&self) {
		self.real.LeaveStandby();
		info!("waking up base stations");
		for station in self.stations.lock().expect("lock").iter_mut() {
			station.send(StationCommand::SetState(StationState::On))
		}
	}
}

pub static SERVER_TRACKED_DEVICE_PROVIDER: Lazy<WithVtables<ServerTrackedProvider>> =
	Lazy::new(|| {
		info!("intializing server tracker provider");
		let real = unsafe {
			let factory = get_hmd_driver_factory().expect("factory should exist");
			try_vr!(factory(IServerTrackedDeviceProvider_Version))
				.expect("failed to obtain tracked device provider from factory")
		};

		WithVtables::new(ServerTrackedProvider {
			real: unsafe { VtableRef::from_raw(real as *const _) },
			stations: Mutex::new(vec![]),
			standby_state: Mutex::new(StationState::Unknown),
		})
	});
