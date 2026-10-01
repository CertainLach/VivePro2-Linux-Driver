use std::ffi::{c_char, CStr};

use vive_hid::HmdStatus;

use real_c_string::real_c_string;

use crate::{
	openvr::{
		k_ulInvalidInputComponentHandle, IVRDriverInput, PropertyContainerHandle_t,
		VRInputComponentHandle_t,
	},
	settings::DRIVER_INPUT,
	try_vrinput, Result,
};

const COMPONENTS: [&CStr; 5] = [
	c"/proximity",
	c"/input/system/click",
	c"/input/volume_up/click",
	c"/input/volume_down/click",
	c"/input/mute_mic/click",
];

struct Component<T> {
	name: *const c_char,
	handle: VRInputComponentHandle_t,
	value: Option<T>,
}

pub struct HmdInput {
	comp_proximity: Component<bool>,
	comp_system: Component<bool>,
	comp_volume_up: Component<bool>,
	comp_volume_down: Component<bool>,
	comp_mute_mic: Component<bool>,
}

impl Component<bool> {
	fn new(container: PropertyContainerHandle_t, name: *const c_char) -> Result<Self> {
		let mut handle = k_ulInvalidInputComponentHandle;
		try_vrinput!(DRIVER_INPUT.CreateBooleanComponent(container, name, &mut handle))?;
		if handle == k_ulInvalidInputComponentHandle {
			return Err(crate::Error::Internal("input component handle is invalid"));
		}
		Ok(Self {
			name,
			handle,
			value: None,
		})
	}
	fn update(&self, value: bool) -> Result<()> {
		try_vrinput!(DRIVER_INPUT.UpdateBooleanComponent(self.handle, value, 0.0))?;
		Ok(())
	}
}

impl HmdInput {
	pub fn new(container: PropertyContainerHandle_t) -> Result<Self> {
		Ok(Self {
			comp_proximity: Component::new(container, real_c_string!("/proximity"))?,
			comp_system: Component::new(container, real_c_string!("/input/system/click"))?,
			comp_volume_up: Component::new(container, real_c_string!("/input/volume_up/click"))?,
			comp_volume_down: Component::new(
				container,
				real_c_string!("/input/volume_down/click"),
			)?,
			comp_mute_mic: Component::new(container, real_c_string!("/input/mute_mic/click"))?,
		})
	}

	pub fn update(&mut self, status: &HmdStatus) -> Result<()> {
		self.comp_proximity.update(status.worn())?;
		self.comp_system.update(status.system)?;
		self.comp_volume_up.update(status.volume_up)?;
		self.comp_volume_down.update(status.volume_down)?;
		self.comp_mute_mic.update(status.mute_mic)?;

		Ok(())
	}
}
