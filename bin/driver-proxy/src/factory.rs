use std::{
	ffi::{c_void, CStr},
	os::raw::c_char,
	ptr::null,
};

use crate::{
	log::LogWriter, server_tracked_provider::SERVER_TRACKED_DEVICE_PROVIDER, Error, Result,
};
use cppvtbl::{HasVtable, VtableRef};
use libloading::{Library, Symbol};
use once_cell::sync::{Lazy, OnceCell};
use tokio::runtime::Runtime;
use tracing::info;

use crate::openvr::{
	EVRInitError, IServerTrackedDeviceProviderVtable, IServerTrackedDeviceProvider_Version,
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

pub static TOKIO_RUNTIME: Lazy<Runtime> =
	Lazy::new(|| Runtime::new().expect("tokio init should not fail"));

fn HmdDriverFactory_impl(iface: *const c_char) -> Result<*const c_void> {
	// May be already installed
	if tracing_subscriber::fmt()
		.without_time()
		.with_writer(LogWriter::default)
		.try_init()
		.is_ok()
	{
		// This magic string is also used for installation detection!
		info!("https://patreon.com/0lach");
	}

	let ifacen = unsafe { CStr::from_ptr(iface) };
	info!("requested interface: {ifacen:?}");

	if ifacen == unsafe { CStr::from_ptr(IServerTrackedDeviceProvider_Version) } {
		Ok(
			VtableRef::into_raw(HasVtable::<IServerTrackedDeviceProviderVtable>::get(
				&SERVER_TRACKED_DEVICE_PROVIDER,
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
