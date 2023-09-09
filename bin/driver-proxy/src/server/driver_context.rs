use std::{
	ffi::{c_void, CStr},
	os::raw::c_char,
};

use crate::{driver_host::DRIVER_HOST, openvr::IVRServerDriverHostVtable, try_vr, Result};
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
			info!("hooked!");
			VtableRef::into_raw(HasVtable::<IVRServerDriverHostVtable>::get(&*DRIVER_HOST))
				as *mut _
		} else {
			self.real.GetGenericInterface(pchInterfaceVersion, result)
		}
	}

	fn GetDriverHandle(&self) -> DriverHandle_t {
		self.real.GetDriverHandle()
	}
}

pub static DRIVER_CONTEXT: OnceCell<WithVtables<DriverContext>> = OnceCell::new();

pub fn try_init_driver_context(real: &'static VtableRef<IVRDriverContextVtable>) {
	if DRIVER_CONTEXT.get().is_some() {
		return;
	}
	let new_ctx = WithVtables::new(DriverContext { real });
	DRIVER_CONTEXT
		.set(new_ctx)
		.map_err(|_| ())
		.expect("context is not set");
}
