use std::{
	ffi::{c_void, CStr},
	os::raw::c_char,
	ptr::null,
};

use crate::{Error, Result};
use cppvtbl::{HasVtable, VtableRef};
use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use tracing::info;

use crate::{
	openvr::{
		EVRInitError, IServerTrackedDeviceProviderVtable, IServerTrackedDeviceProvider_Version,
	},
	server_tracked_provider::get_server_tracked_provider,
};

pub type HmdDriverFactory =
	unsafe extern "C" fn(*const c_char, result: *mut EVRInitError) -> *const c_void;
static HMD_DRIVER_FACTORY: OnceCell<Symbol<HmdDriverFactory>> = OnceCell::new();
pub fn get_hmd_driver_factory() -> Result<&'static Symbol<'static, HmdDriverFactory>> {
	HMD_DRIVER_FACTORY.get_or_try_init(|| {
		let mut path =
			process_path::get_dylib_path().ok_or(Error::Internal("process path failed"))?;
		path.pop();
		path.push("driver_lighthouse_real.so");

		let library: &'static mut Library =
			Box::leak(Box::new(unsafe { libloading::Library::new(&path)? }));
		Ok(unsafe { library.get(b"HmdDriverFactory") }.expect("can't find HmdDriverFactory"))
	})
}

fn HmdDriverFactory_impl(iface: *const c_char) -> Result<*const c_void> {
	// May be already installed
	let _ = tracing_subscriber::fmt().without_time().try_init();

	let ifacen = unsafe { CStr::from_ptr(iface) };
	info!("requested interface: {ifacen:?}");

	if ifacen == unsafe { CStr::from_ptr(IServerTrackedDeviceProvider_Version) } {
		let provider = get_server_tracked_provider()?;
		Ok(
			VtableRef::into_raw(HasVtable::<IServerTrackedDeviceProviderVtable>::get(
				&provider,
			)) as *const _ as *const c_void,
		)
	} else {
		let factory = get_hmd_driver_factory()?;
		unsafe { try_vr!(factory(iface)) }
	}
}

#[no_mangle]
pub extern "C" fn HmdDriverFactory(
	iface: *const c_char,
	result: *mut EVRInitError,
) -> *const c_void {
	eprintln!("factory call");
	vr_result!(result, HmdDriverFactory_impl(iface), null())
}
