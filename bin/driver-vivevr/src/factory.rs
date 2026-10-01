use std::{
	ffi::{c_void, CStr},
	os::raw::c_char,
	ptr::null,
};

use crate::{
	log::LogWriter, server_tracked_provider::SERVER_TRACKED_DEVICE_PROVIDER, Error, Result,
};
use cppvtbl::{HasVtable, VtableRef};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use tracing::info;

use crate::openvr::{
	EVRInitError, IServerTrackedDeviceProviderVtable, IServerTrackedDeviceProvider_Version,
};

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
		Err(Error::VR(EVRInitError::VRInitError_Init_InterfaceNotFound))
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
