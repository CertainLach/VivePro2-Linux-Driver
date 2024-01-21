use std::ffi::{c_void, CString};
use std::{marker::PhantomData, os::raw::c_char};

use cppvtbl::VtableRef;
use once_cell::sync::Lazy;
use openvr::{
	k_unBoolPropertyTag, k_unFloatPropertyTag, EPropertyWriteType, ETrackedDeviceProperty,
	ETrackedPropertyError, IVRProperties, IVRPropertiesVtable, IVRProperties_Version,
	PropertyWrite_t,
};
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
				let mut err = EVRSettingsError::VRSettingsError_None;
				let v = SETTINGS.$get_meth(self.0, self.1, &mut err);
				if err != EVRSettingsError::VRSettingsError_None {
					error!("failed: {:?}", err);
					return $default;
				}
				v
			}
			#[instrument]
			pub fn set(&self, value: $ty) {
				let mut err = EVRSettingsError::VRSettingsError_None;
				SETTINGS.$set_meth(self.0, self.1, value, &mut err);
				if err != EVRSettingsError::VRSettingsError_None {
					error!("failed: {:?}", err);
				}
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

pub enum PropertyValue {
	Float(f32),
	FloatArray(Vec<f32>),
	Bool(bool),
}
impl PropertyValue {
	fn tag(&self) -> u32 {
		match self {
			Self::Float(_) | Self::FloatArray(_) => k_unFloatPropertyTag,
			Self::Bool(_) => k_unBoolPropertyTag,
		}
	}
	fn size(&self) -> u32 {
		match self {
			PropertyValue::Float(_) => 4,
			PropertyValue::FloatArray(v) => 4 * v.len() as u32,
			PropertyValue::Bool(_) => 1,
		}
	}
	fn buf(&mut self) -> *mut c_void {
		match self {
			PropertyValue::Float(f) => (f as *mut f32).cast(),
			PropertyValue::FloatArray(f) => f.as_mut_ptr().cast(),
			PropertyValue::Bool(v) => (v as *mut bool).cast(),
		}
	}
}
pub struct Property {
	name: ETrackedDeviceProperty,
	value: PropertyValue,
}
impl Property {
	pub fn new(name: ETrackedDeviceProperty, value: PropertyValue) -> Self {
		Self { name, value }
	}
}
pub fn set_properties(container: u64, mut props: Vec<Property>) {
	let mut batch = Vec::with_capacity(props.len());
	for prop in props.iter_mut() {
		batch.push(PropertyWrite_t {
			writeType: EPropertyWriteType::PropertyWrite_Set,
			prop: prop.name,
			unTag: prop.value.tag(),
			unBufferSize: prop.value.size(),
			pvBuffer: prop.value.buf(),

			eError: ETrackedPropertyError::TrackedProp_Success,
			eSetError: ETrackedPropertyError::TrackedProp_Success,
		});
	}
	PROPERTIES.WritePropertyBatch(container, batch.as_mut_ptr(), batch.len() as u32);
}

pub static PROPERTIES: Lazy<&'static VtableRef<IVRPropertiesVtable>> = Lazy::new(|| {
	let ctx = DRIVER_CONTEXT
		.get()
		.expect("context should be initialized at this point");
	let raw = ctx
		.get_generic_interface(IVRProperties_Version)
		.expect("there should be properties interface");
	unsafe { VtableRef::from_raw(raw as *const VtableRef<IVRPropertiesVtable>) }
});
