use std::ffi::CString;
use std::{marker::PhantomData, os::raw::c_char};

use cppvtbl::VtableRef;
use once_cell::sync::Lazy;
use tracing::{error, instrument};

use crate::driver_context::DRIVER_CONTEXT;
use crate::openvr::{EVRSettingsError, IVRSettings, IVRSettingsVtable, IVRSettings_Version};
use crate::{Error, Result};

#[derive(Debug)]
pub struct Setting<T>(*const c_char, *const c_char, PhantomData<T>);
impl<T> Setting<T> {
	pub const fn unsafe_new(section: *const c_char, name: *const c_char) -> Self {
		Self(section, name, PhantomData)
	}
}
macro_rules! impl_setting {
	($ty:ty, $get_meth:ident, $set_meth:ident, $default:expr) => {
		#[allow(dead_code)]
		impl Setting<$ty> {
			#[instrument]
			pub fn get(&self) -> $ty {
				let err: Result<()> = try {
					let mut err = EVRSettingsError::VRSettingsError_None;
					return SETTINGS.$get_meth(self.0, self.1, &mut err);
				};
				error!("failed: {}", err.err().unwrap());
				$default
			}
			#[instrument]
			pub fn set(&self, value: $ty) {
				let err: Result<()> = try {
					let mut err = EVRSettingsError::VRSettingsError_None;
					SETTINGS.$set_meth(self.0, self.1, value, &mut err);
					return;
				};
				error!("failed: {}", err.err().unwrap());
			}
		}
	};
}
impl_setting!(i32, GetInt32, SetInt32, 0);
impl_setting!(bool, GetBool, SetBool, false);

const STRING_SIZE: usize = 65535;
impl Setting<String> {
	#[instrument]
	pub fn get(&self) -> String {
		let err: Result<()> = try {
			let mut err = EVRSettingsError::VRSettingsError_None;
			let mut buf = vec![0u8; STRING_SIZE];
			SETTINGS.GetString(
				self.0,
				self.1,
				buf.as_mut_ptr().cast(),
				STRING_SIZE as u32,
				&mut err,
			);

			if err == EVRSettingsError::VRSettingsError_None {
				buf.truncate(buf.iter().position(|&c| c == 0).unwrap_or(buf.len()));

				return String::from_utf8(buf)
					.map_err(|_| Error::Internal("setting value is not utf-8"))?;
			};
			Err(Error::Internal("failed to get string"))?;
		};
		error!("failed: {}", err.err().unwrap());
		"".to_owned()
	}
	#[instrument]
	pub fn set(&self, value: String) {
		let err: Result<()> = try {
			let cstring =
				CString::new(value).map_err(|_| Error::Internal("setting value contains \\0"))?;
			let mut err = EVRSettingsError::VRSettingsError_None;
			SETTINGS.SetString(self.0, self.1, cstring.as_ptr(), &mut err);
			return;
		};
		error!("failed: {}", err.err().unwrap());
	}
}

#[macro_export]
macro_rules! setting {
	($section:expr, $name:expr) => {
		crate::settings::Setting::unsafe_new(
			::real_c_string::real_c_string!($section),
			::real_c_string::real_c_string!($name),
		)
	};
}

pub static SETTINGS: Lazy<&'static VtableRef<IVRSettingsVtable>> = Lazy::new(|| {
	let ctx = DRIVER_CONTEXT
		.get()
		.expect("context should be initialized at this point");
	let raw = ctx
		.get_generic_interface(IVRSettings_Version)
		.expect("there should be settings interface");
	unsafe { VtableRef::from_raw(raw as *const VtableRef<IVRSettingsVtable>) }
});
