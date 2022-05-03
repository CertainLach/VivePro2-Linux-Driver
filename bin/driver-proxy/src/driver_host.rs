use cppvtbl::{impl_vtables, HasVtable, VtableRef, WithVtables};
use lens_client::start_lens_server;
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::rc::Rc;
use tracing::{error, info};
use vive_hid::{Resolution, SteamDevice, ViveDevice};

use crate::driver_context::get_driver_context;
use crate::hmd::HmdDriver;
use crate::openvr::{
	Compositor_FrameTiming, DriverPose_t, ETrackedDeviceClass, EVREventType, HmdMatrix34_t,
	HmdRect2_t, ITrackedDeviceServerDriverVtable, IVRServerDriverHost_Version, TrackedDevicePose_t,
	VREvent_t,
};
use crate::openvr::{IVRServerDriverHost, IVRServerDriverHostVtable, VREvent_Data_t};
use crate::settings::Setting;
use crate::{setting, Result};

#[impl_vtables(IVRServerDriverHost)]
pub struct DriverHost {
	real: &'static VtableRef<IVRServerDriverHostVtable>,
}

const HMD_RESOLUTION: Setting<i32> = setting!("vivepro2", "resolution");
const BRIGHTNESS: Setting<i32> = setting!("vivepro2", "brightness");
const NOISE_CANCEL: Setting<bool> = setting!("vivepro2", "noiseCancel");

impl IVRServerDriverHost for DriverHost {
	fn TrackedDeviceAdded(
		&self,
		pchDeviceSerialNumber: *const c_char,
		eDeviceClass: ETrackedDeviceClass,
		pDriver: *const VtableRef<ITrackedDeviceServerDriverVtable>,
	) -> bool {
		let sn = unsafe { CStr::from_ptr(pchDeviceSerialNumber) }
			.to_string_lossy()
			.to_string();
		info!("added tracked device: {sn:?} ({eDeviceClass:?})");
		if eDeviceClass == ETrackedDeviceClass::TrackedDeviceClass_HMD {
			let err: Result<()> = try {
				// Steam part is opened for checking if this is really a needed HMD device
				let _steam = Rc::new(SteamDevice::open(&sn)?);
				// We don't know for sure this device serial
				let vive = Rc::new(ViveDevice::open_first()?);

				let resolution = {
					let res = HMD_RESOLUTION.get();
					let resolution =
						Resolution::try_from(res as u8).unwrap_or(Resolution::R2448x1224f90);
					HMD_RESOLUTION.set(resolution as u8 as i32);

					vive.set_resolution(resolution)?;
					resolution
				};
				{
					let nc = NOISE_CANCEL.get();
					NOISE_CANCEL.set(nc);

					vive.toggle_noise_canceling(nc)?;
				}
				{
					let mut brightness = BRIGHTNESS.get();
					if brightness == 0 {
						brightness = 130;
					}
					BRIGHTNESS.set(brightness);

					vive.set_brightness(brightness as u8)?;
				}

				let config = vive.read_config()?;

				let lens = Rc::new(RefCell::new(start_lens_server(
					config.inhouse_lens_correction,
				)?));
				let real = unsafe { VtableRef::from_raw(pDriver) };

				let hmd = Box::leak(Box::new(WithVtables::new(HmdDriver {
					// steam,
					// vive,
					lens,
					real,
					resolution,
				})));

				return self.real.TrackedDeviceAdded(
					pchDeviceSerialNumber,
					eDeviceClass,
					HasVtable::<ITrackedDeviceServerDriverVtable>::get(hmd),
				);
			};
			error!("failed to wrap hmd: {}", err.err().unwrap());
		}
		self.real
			.TrackedDeviceAdded(pchDeviceSerialNumber, eDeviceClass, pDriver)
	}

	fn TrackedDevicePoseUpdated(
		&self,
		unWhichDevice: u32,
		newPose: *const DriverPose_t,
		unPoseStructSize: u32,
	) {
		self.real
			.TrackedDevicePoseUpdated(unWhichDevice, newPose, unPoseStructSize)
	}

	fn VsyncEvent(&self, vsyncTimeOffsetSeconds: f64) {
		self.real.VsyncEvent(vsyncTimeOffsetSeconds)
	}

	fn VendorSpecificEvent(
		&self,
		unWhichDevice: u32,
		eventType: EVREventType,
		eventData: *const VREvent_Data_t,
		eventTimeOffset: f64,
	) {
		self.real
			.VendorSpecificEvent(unWhichDevice, eventType, eventData, eventTimeOffset)
	}

	fn IsExiting(&self) -> bool {
		self.real.IsExiting()
	}

	fn PollNextEvent(&self, pEvent: *mut VREvent_t, uncbVREvent: u32) -> bool {
		self.real.PollNextEvent(pEvent, uncbVREvent)
	}

	fn GetRawTrackedDevicePoses(
		&self,
		fPredictedSecondsFromNow: f32,
		pTrackedDevicePoseArray: *mut TrackedDevicePose_t,
		unTrackedDevicePoseArrayCount: u32,
	) {
		self.real.GetRawTrackedDevicePoses(
			fPredictedSecondsFromNow,
			pTrackedDevicePoseArray,
			unTrackedDevicePoseArrayCount,
		)
	}

	fn RequestRestart(
		&self,
		pchLocalizedReason: *const c_char,
		pchExecutableToStart: *const c_char,
		pchArguments: *const c_char,
		pchWorkingDirectory: *const c_char,
	) {
		self.real.RequestRestart(
			pchLocalizedReason,
			pchExecutableToStart,
			pchArguments,
			pchWorkingDirectory,
		)
	}

	fn GetFrameTimings(&self, pTiming: *mut Compositor_FrameTiming, nFrames: u32) -> u32 {
		self.real.GetFrameTimings(pTiming, nFrames)
	}

	fn SetDisplayEyeToHead(
		&self,
		unWhichDevice: u32,
		eyeToHeadLeft: *const HmdMatrix34_t,
		eyeToHeadRight: *const HmdMatrix34_t,
	) {
		self.real
			.SetDisplayEyeToHead(unWhichDevice, eyeToHeadLeft, eyeToHeadRight)
	}

	fn SetDisplayProjectionRaw(
		&self,
		unWhichDevice: u32,
		eyeLeft: *const HmdRect2_t,
		eyeRight: *const HmdRect2_t,
	) {
		self.real
			.SetDisplayProjectionRaw(unWhichDevice, eyeLeft, eyeRight)
	}

	fn SetRecommendedRenderTargetSize(&self, unWhichDevice: u32, nWidth: u32, nHeight: u32) {
		self.real
			.SetRecommendedRenderTargetSize(unWhichDevice, nWidth, nHeight)
	}
}

static DRIVER_HOST: OnceCell<WithVtables<DriverHost>> = OnceCell::new();

pub fn get_driver_host() -> Result<&'static WithVtables<DriverHost>> {
	DRIVER_HOST.get_or_try_init(|| {
		let context = get_driver_context()?;
		let real = unsafe {
			&*(context.get_generic_interface(IVRServerDriverHost_Version)? as *const _
				as *const VtableRef<IVRServerDriverHostVtable>)
		};
		Ok(WithVtables::new(DriverHost { real }))
	})
}
