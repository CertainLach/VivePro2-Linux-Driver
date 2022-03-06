use std::{marker::PhantomData, os::raw::c_char};

use cppvtbl::VtableRef;
use once_cell::sync::OnceCell;
use tracing::{error, instrument};

use crate::driver_context::get_driver_context;
use crate::openvr::{EVRSettingsError, IVRSettings, IVRSettingsVtable, IVRSettings_Version};
use crate::Result;

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
					let settings = get_settings()?;
					let mut err = EVRSettingsError::VRSettingsError_None;
					return settings.$get_meth(self.0, self.1, &mut err);
				};
				error!("failed: {}", err.err().unwrap());
				$default
			}
			#[instrument]
			pub fn set(&self, value: $ty) {
				let err: Result<()> = try {
					let settings = get_settings()?;
					let mut err = EVRSettingsError::VRSettingsError_None;
					settings.$set_meth(self.0, self.1, value, &mut err);
					return;
				};
				error!("failed: {}", err.err().unwrap());
			}
		}
	};
}
impl_setting!(i32, GetInt32, SetInt32, 0);
impl_setting!(bool, GetBool, SetBool, false);

#[macro_export]
macro_rules! setting {
	($section:expr, $name:expr) => {
		crate::settings::Setting::unsafe_new(
			::real_c_string::real_c_string!($section),
			::real_c_string::real_c_string!($name),
		)
	};
}

static SETTINGS: OnceCell<&'static VtableRef<IVRSettingsVtable>> = OnceCell::new();
pub fn get_settings() -> Result<&'static &'static VtableRef<IVRSettingsVtable>> {
	SETTINGS.get_or_try_init(|| {
		let ctx = get_driver_context()?;
		let raw = ctx.get_generic_interface(IVRSettings_Version)?;
		Ok(unsafe { VtableRef::from_raw(raw as *const VtableRef<IVRSettingsVtable>) })
	})
}
