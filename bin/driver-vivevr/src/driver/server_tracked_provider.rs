use std::{os::raw::c_char, ptr::null, sync::Mutex};

use crate::{
	driver_context::{get_interface, try_init_driver_context},
	factory::TOKIO_RUNTIME,
	hmd::prepare_hmd,
	log::try_init_driver_log,
	setting,
	settings::Setting,
};
use cppvtbl::{impl_vtables, VtableRef, WithVtables};
use once_cell::sync::Lazy;
use openvr::IVRDriverLog_Version;
use tokio::task::LocalSet;
use tracing::{error, info};
use valve_pm::{start_manager, StationCommand, StationControl, StationState};

use crate::openvr::{
	EVRInitError, IServerTrackedDeviceProvider, IServerTrackedDeviceProviderVtable,
	IServerTrackedDeviceProvider_Version, ITrackedDeviceServerDriver_Version,
	IVRCameraComponent_Version, IVRCompositorPluginProvider_Version, IVRDisplayComponent_Version,
	IVRDriverContextVtable, IVRDriverDirectModeComponent_Version, IVRDriverManager_Version,
	IVRResources_Version, IVRSettings_Version, IVRVirtualDisplay_Version,
	IVRWatchdogProvider_Version,
};

struct InterfaceVersions([*const c_char; 12]);
unsafe impl Sync for InterfaceVersions {}
static INTERFACE_VERSIONS: InterfaceVersions = InterfaceVersions([
	IVRSettings_Version,
	ITrackedDeviceServerDriver_Version,
	IVRDisplayComponent_Version,
	IVRDriverDirectModeComponent_Version,
	IVRCameraComponent_Version,
	IServerTrackedDeviceProvider_Version,
	IVRWatchdogProvider_Version,
	IVRVirtualDisplay_Version,
	IVRDriverManager_Version,
	IVRResources_Version,
	IVRCompositorPluginProvider_Version,
	null(),
]);

// (name ":" "BS2" ":" "0"/"1") ** ","
const BASE_STATIONS: Setting<String> = setting!("driver_lighthouse", "PowerManagedBaseStations2");
// 0 - disabled
// 1 - sleep
// 2 - standby
const POWER_MANAGEMENT: Setting<i32> = setting!("vivepro2", "basestationPowerManagement");

#[impl_vtables(IServerTrackedDeviceProvider)]
pub struct ServerTrackedProvider {
	stations: Mutex<Vec<StationControl>>,
	standby_state: Mutex<StationState>,
}
impl IServerTrackedDeviceProvider for ServerTrackedProvider {
	fn Init(
		&self,
		pDriverContext: *const cppvtbl::VtableRef<IVRDriverContextVtable>,
	) -> EVRInitError {
		try_init_driver_context(unsafe { &*pDriverContext });
		try_init_driver_log(
			get_interface(IVRDriverLog_Version).expect("always able to initialize driver log"),
		);

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

		if let Err(err) = prepare_hmd() {
			error!("failed to prepare vive pro 2 display: {err}");
		}
		EVRInitError::VRInitError_None
	}

	fn Cleanup(&self) {
		info!("disconnecting from base stations");
		let _runtime = TOKIO_RUNTIME.enter();
		let localset = LocalSet::new();
		for station in self.stations.lock().expect("lock").drain(..) {
			localset.spawn_local(station.finish());
		}
		TOKIO_RUNTIME.block_on(localset);
	}

	fn GetInterfaceVersions(&self) -> *const *const c_char {
		INTERFACE_VERSIONS.0.as_ptr()
	}

	fn RunFrame(&self) {}

	fn ShouldBlockStandbyMode(&self) -> bool {
		false
	}

	fn EnterStandby(&self) {
		info!("making station standby");
		for station in self.stations.lock().expect("lock").iter_mut() {
			station.send(StationCommand::SetState(
				*self.standby_state.lock().expect("lock"),
			))
		}
	}

	fn LeaveStandby(&self) {
		info!("waking up base stations");
		for station in self.stations.lock().expect("lock").iter_mut() {
			station.send(StationCommand::SetState(StationState::On))
		}
	}
}

pub static SERVER_TRACKED_DEVICE_PROVIDER: Lazy<WithVtables<ServerTrackedProvider>> =
	Lazy::new(|| {
		info!("intializing server tracker provider");
		WithVtables::new(ServerTrackedProvider {
			stations: Mutex::new(vec![]),
			standby_state: Mutex::new(StationState::Unknown),
		})
	});
