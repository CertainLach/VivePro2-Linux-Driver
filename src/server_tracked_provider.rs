use std::os::raw::c_char;

use crate::{
	driver_context::{get_driver_context, try_init_driver_context},
	factory::get_hmd_driver_factory,
	Result,
};
use cppvtbl::{impl_vtables, HasVtable, VtableRef, WithVtables};
use once_cell::sync::OnceCell;
use tracing::info;

use crate::openvr::{
	EVRInitError, IServerTrackedDeviceProvider, IServerTrackedDeviceProviderVtable,
	IServerTrackedDeviceProvider_Version, IVRDriverContextVtable,
};

#[impl_vtables(IServerTrackedDeviceProvider)]
pub struct ServerTrackedProvider {
	real: &'static VtableRef<IServerTrackedDeviceProviderVtable>,
}
impl IServerTrackedDeviceProvider for ServerTrackedProvider {
	fn Init(
		&self,
		pDriverContext: *const cppvtbl::VtableRef<IVRDriverContextVtable>,
	) -> EVRInitError {
		let _ = try_init_driver_context(unsafe { &*pDriverContext });
		let context = get_driver_context().expect("context just initialized");
		self.real.Init(
			VtableRef::into_raw(HasVtable::<IVRDriverContextVtable>::get(context)) as *const _,
		)
	}

	fn Cleanup(&self) {
		self.real.Cleanup()
	}

	fn GetInterfaceVersions(&self) -> *const *const c_char {
		self.real.GetInterfaceVersions()
	}

	fn RunFrame(&self) {
		self.real.RunFrame()
	}

	fn ShouldBlockStandbyMode(&self) -> bool {
		self.real.ShouldBlockStandbyMode()
	}

	fn EnterStandby(&self) {
		self.real.EnterStandby()
	}

	fn LeaveStandby(&self) {
		self.real.LeaveStandby()
	}
}

static SERVER_TRACKED_DEVICE_PROVIDER: OnceCell<WithVtables<ServerTrackedProvider>> =
	OnceCell::new();
pub fn get_server_tracked_provider() -> Result<&'static WithVtables<ServerTrackedProvider>> {
	SERVER_TRACKED_DEVICE_PROVIDER.get_or_try_init(|| {
		info!("intializing server tracker provider");
		let real = unsafe {
			let factory = get_hmd_driver_factory()?;
			try_vr!(factory(IServerTrackedDeviceProvider_Version))?
		};
		Ok(WithVtables::new(ServerTrackedProvider {
			real: unsafe { VtableRef::from_raw(real as *const _) },
		})) as Result<_>
	})
}
