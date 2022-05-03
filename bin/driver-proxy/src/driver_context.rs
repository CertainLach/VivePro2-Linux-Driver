use std::{
	ffi::{c_void, CStr},
	os::raw::c_char,
	ptr::null_mut,
};

use crate::{
	driver_host::get_driver_host, openvr::IVRServerDriverHostVtable, try_vr, vr_result, Error,
	Result,
};
use cppvtbl::{impl_vtables, HasVtable, VtableRef, WithVtables};
use once_cell::sync::OnceCell;
use tracing::info;

use crate::openvr::{
	DriverHandle_t, EVRInitError, IVRDriverContext, IVRDriverContextVtable,
	IVRServerDriverHost_Version,
};

#[impl_vtables(IVRDriverContext)]
pub struct DriverContext {
	pub real: &'static VtableRef<IVRDriverContextVtable>,
}
impl DriverContext {
	pub fn get_generic_interface(&self, name: *const c_char) -> Result<*mut c_void> {
		try_vr!(self.real.GetGenericInterface(name))
	}
}

impl IVRDriverContext for DriverContext {
	fn GetGenericInterface(
		&self,
		pchInterfaceVersion: *const c_char,
		result: *mut EVRInitError,
	) -> *mut c_void {
		let name = unsafe { CStr::from_ptr(pchInterfaceVersion) };
		info!("get generic interface {name:?}");
		if name == unsafe { CStr::from_ptr(IVRServerDriverHost_Version) } {
			vr_result!(
				result,
				get_driver_host().map(|host| {
					VtableRef::into_raw(HasVtable::<IVRServerDriverHostVtable>::get(host)) as *mut _
				}),
				null_mut()
			)
		} else {
			self.real.GetGenericInterface(pchInterfaceVersion, result)
		}
	}

	fn GetDriverHandle(&self) -> DriverHandle_t {
		self.real.GetDriverHandle()
	}
}

static DRIVER_CONTEXT: OnceCell<WithVtables<DriverContext>> = OnceCell::new();
pub fn try_init_driver_context(real: &'static VtableRef<IVRDriverContextVtable>) -> Result<(), ()> {
	if DRIVER_CONTEXT.get().is_some() {
		return Ok(());
	}
	let context = WithVtables::new(DriverContext { real });
	DRIVER_CONTEXT.set(context).map_err(|_| ())
}

pub fn get_driver_context() -> Result<&'static WithVtables<DriverContext>> {
	DRIVER_CONTEXT
		.get()
		.ok_or_else(|| Error::Internal("driver context is not initialized yet"))
}
