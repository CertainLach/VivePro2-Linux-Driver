use std::{ffi::c_void, os::raw::c_char};

use cppvtbl::VtableRef;
use once_cell::sync::OnceCell;

use crate::openvr::{IVRDriverContext, IVRDriverContextVtable};
use crate::{try_vr, Error, Result};

pub struct DriverContext {
	real: &'static VtableRef<IVRDriverContextVtable>,
}
impl DriverContext {
	pub fn get_generic_interface(&self, name: *const c_char) -> Result<*mut c_void> {
		try_vr!(self.real.GetGenericInterface(name))
	}
}

pub static DRIVER_CONTEXT: OnceCell<DriverContext> = OnceCell::new();

pub fn try_init_driver_context(real: &'static VtableRef<IVRDriverContextVtable>) {
	let _ = DRIVER_CONTEXT.set(DriverContext { real });
}

pub fn get_interface<V>(version: *const c_char) -> Result<&'static VtableRef<V>> {
	let context = DRIVER_CONTEXT
		.get()
		.ok_or(Error::Internal("driver context is not initialized"))?;
	let raw = context.get_generic_interface(version)?;
	if raw.is_null() {
		return Err(Error::Internal("interface is not available"));
	}
	Ok(unsafe { VtableRef::from_raw(raw as *const VtableRef<V>) })
}
