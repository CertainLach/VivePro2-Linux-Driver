use std::{
	env::var_os,
	ffi::{c_void, CStr, OsString},
	os::raw::c_char,
	process::Command,
	rc::Rc,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, OnceLock,
	},
	thread,
	time::{Duration, Instant},
};

use crate::{
	driver::{hmd_audio::HmdAudioKeys, hmd_input::HmdInput},
	driver_host::{hook_tracked_device_added, real_tracked_device_added},
	setting,
	settings::{set_properties, Property, PropertyValue, Setting, PROPERTIES},
	Error, Result,
};
use cppvtbl::{impl_vtables, HasVtable, VtableRef, WithVtables};
use lens_distortion::{Eye, LensClient, LensLibrary, StubClient};
use openvr::{ETrackedDeviceProperty, HmdVector2_t, IVRProperties, PropertyContainerHandle_t};
use tracing::{error, info, instrument, warn};
use vive_hid::{Mode, SteamDevice, ViveDevice};

use crate::openvr::{
	DistortionCoordinates_t, DriverPose_t, ETrackedDeviceClass, EVREye, EVRInitError,
	ITrackedDeviceServerDriver, ITrackedDeviceServerDriverVtable, IVRDisplayComponent,
	IVRDisplayComponentVtable, IVRDisplayComponent_Version, IVRServerDriverHostVtable,
};

const HMD_RESOLUTION: Setting<i32> = setting!("vivepro2", "resolution");
const BRIGHTNESS: Setting<i32> = setting!("vivepro2", "brightness");
const NOISE_CANCEL: Setting<bool> = setting!("vivepro2", "noiseCancel");

const STATUS_TIMEOUT: Duration = Duration::from_secs(2);

fn map_eye(eye: EVREye) -> Eye {
	match eye {
		EVREye::Eye_Left => Eye::Left,
		EVREye::Eye_Right => Eye::Right,
	}
}

fn set_hmd_properties(container: PropertyContainerHandle_t, mode: Mode) {
	set_properties(
		container,
		vec![
			Property::new(
				ETrackedDeviceProperty::Prop_DisplayFrequency_Float,
				PropertyValue::Float(mode.frame_rate),
			),
			Property::new(
				ETrackedDeviceProperty::Prop_DisplaySupportsMultipleFramerates_Bool,
				PropertyValue::Bool(true),
			),
			Property::new(
				ETrackedDeviceProperty::Prop_SecondsFromVsyncToPhotons_Float,
				PropertyValue::Float((1.0 / mode.frame_rate) + mode.extra_photon_vsync),
			),
			// Property::new(
			// 	ETrackedDeviceProperty::Prop_MinimumIpdStepMeters_Float,
			// 	PropertyValue::Float(0.0005),
			// ),
			// Property::new(
			// 	ETrackedDeviceProperty::Prop_UserIpdMeters_Float,
			// 	// TODO
			// 	PropertyValue::Float(0.0005),
			// ),
			Property::new(
				ETrackedDeviceProperty::Prop_ContainsProximitySensor_Bool,
				PropertyValue::Bool(true),
			),
			Property::new(
				ETrackedDeviceProperty::Prop_UserHeadToEyeDepthMeters_Float,
				PropertyValue::Float(0.015),
			),
			Property::new(
				ETrackedDeviceProperty::Prop_DisplayAvailableFrameRates_Float_Array,
				PropertyValue::FloatArray(if mode.frame_rate == 90.0 {
					vec![90.0, 120.0]
				} else {
					vec![120.0, 90.0]
				}),
			),
			Property::new(
				ETrackedDeviceProperty::Prop_DisplaySupportsRuntimeFramerateChange_Bool,
				PropertyValue::Bool(true),
			),
		],
	);
}

#[impl_vtables(IVRDisplayComponent)]
struct HmdDisplay {
	lens: Rc<dyn LensClient>,
	mode: Mode,
}

impl IVRDisplayComponent for HmdDisplay {
	#[instrument(skip(self))]
	fn GetWindowBounds(&self, pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32) {
		let Mode { width, height, .. } = self.mode;
		unsafe {
			*pnX = 0;
			*pnY = 0;
			*pnWidth = width;
			*pnHeight = height;
		}
	}

	fn IsDisplayOnDesktop(&self) -> bool {
		false
	}

	fn IsDisplayRealDisplay(&self) -> bool {
		true
	}

	fn GetRecommendedRenderTargetSize(&self, pnWidth: *mut u32, pnHeight: *mut u32) {
		let Mode { width, height, .. } = self.mode;
		unsafe {
			*pnWidth = width / 2;
			*pnHeight = height;
		}
	}

	#[instrument(skip(self))]
	fn GetEyeOutputViewport(
		&self,
		eEye: EVREye,
		pnX: *mut u32,
		pnY: *mut u32,
		pnWidth: *mut u32,
		pnHeight: *mut u32,
	) {
		// let err: Result<()> = try {
		let Mode { width, height, .. } = self.mode;
		unsafe {
			*pnX = if eEye == EVREye::Eye_Left {
				0
			} else {
				width / 2
			};
			*pnY = 0;
			*pnWidth = width / 2;
			*pnHeight = height;
		}
		// 	return;
		// };
		// error!("failed: {}", err.err().unwrap());
		// self.real
		// 	.GetEyeOutputViewport(eEye, pnX, pnY, pnWidth, pnHeight)
	}

	#[instrument(skip(self))]
	fn GetProjectionRaw(
		&self,
		eEye: EVREye,
		pfLeft: *mut f32,
		pfRight: *mut f32,
		pfTop: *mut f32,
		pfBottom: *mut f32,
	) {
		let project = |lens: &dyn LensClient| -> Result<()> {
			let result = lens.project(map_eye(eEye))?;
			unsafe {
				*pfLeft = result.left;
				*pfRight = result.right;
				if lens.matrix_needs_inversion()? {
					*pfTop = result.bottom;
					*pfBottom = result.top;
				} else {
					*pfTop = result.top;
					*pfBottom = result.bottom;
				}
			}
			Ok(())
		};
		if let Err(err) = project(&*self.lens) {
			error!("failed: {}", err);
			project(&StubClient).expect("stub projection never fails");
		}
	}

	#[instrument(skip(self))]
	fn ComputeDistortion(&self, eEye: EVREye, fU: f32, fV: f32) -> DistortionCoordinates_t {
		let distort = |lens: &dyn LensClient| -> Result<DistortionCoordinates_t> {
			let inverse = lens.matrix_needs_inversion()?;
			let result = lens.distort(map_eye(eEye), [fU, if inverse { 1.0 - fV } else { fV }])?;
			Ok(DistortionCoordinates_t {
				rfRed: result.red,
				rfGreen: result.green,
				rfBlue: result.blue,
			})
		};
		match distort(&*self.lens) {
			Ok(v) => v,
			Err(err) => {
				error!("failed: {}", err);
				distort(&StubClient).expect("stub distortion never fails")
			}
		}
	}

	fn ComputeInverseDistortion(
		&self,
		_idk1: *mut HmdVector2_t,
		_eEye: EVREye,
		_fU: f32,
		_fV: f32,
	) -> i32 {
		// Not entirely sure what should this function do, but original impl has only `xor eax eax; retn` inside,
		// so fine by me. Not delegating to real display, to prevent it from somehow breaking in the future.
		0
	}
}

#[impl_vtables(ITrackedDeviceServerDriver)]
pub struct HmdDriver {
	real: &'static VtableRef<ITrackedDeviceServerDriverVtable>,
	display: *mut c_void,
	mode: Mode,
	status_updater_stop: Arc<AtomicBool>,
}

fn run_status_updater(
	container: PropertyContainerHandle_t,
	mut input: HmdInput,
	stop: &AtomicBool,
) -> Result<()> {
	let vive = ViveDevice::open_first()?;
	vive.set_status_polling(true)?;
	// let (overlay, overlay_ipd) = mpsc::channel();
	// if let Err(err) = thread::Builder::new()
	// 	.name("vive-ipd-overlay".to_owned())
	// 	.spawn(move || run_ipd_overlay(overlay_ipd))
	// {
	// 	warn!("failed to spawn ipd overlay: {err}");
	// }
	let mut audio_keys = HmdAudioKeys::default();
	let mut last_ipd = 0;
	let mut last_status = Instant::now();
	while !stop.load(Ordering::Relaxed) {
		match vive.read_status(500)? {
			Some(status) => {
				last_status = Instant::now();
				if let Err(e) = input.update(&status) {
					warn!("hmd input update failed: {e}")
				}
				audio_keys.update(status);
				if status.ipd != 0 && status.ipd != last_ipd {
					// let initial = last_ipd == 0;
					last_ipd = status.ipd;
					info!("ipd changed: {}m", status.ipd_meters());
					set_properties(
						container,
						vec![Property::new(
							ETrackedDeviceProperty::Prop_UserIpdMeters_Float,
							PropertyValue::Float(status.ipd_meters()),
						)],
					);
					// if !initial {
					// 	let _ = overlay.send(status.ipd_meters());
					// }
				}
			}
			None if last_status.elapsed() > STATUS_TIMEOUT => {
				warn!("no hmd status reports, re-enabling polling");
				vive.set_status_polling(true)?;
				last_status = Instant::now();
			}
			None => {}
		}
	}
	Ok(())
}

impl ITrackedDeviceServerDriver for HmdDriver {
	fn Activate(&self, unObjectId: u32) -> EVRInitError {
		let res = self.real.Activate(unObjectId);
		if res != EVRInitError::VRInitError_None {
			return res;
		}
		info!("hmd activated as device {unObjectId}");
		let container = PROPERTIES.TrackedDeviceToPropertyContainer(unObjectId);
		set_hmd_properties(container, self.mode);

		self.status_updater_stop.store(false, Ordering::Relaxed);
		let stop = self.status_updater_stop.clone();
		if let Err(err) = thread::Builder::new()
			.name("vive-status".to_owned())
			.spawn(move || {
				if let Err(err) = HmdInput::new(container)
					.and_then(|input| run_status_updater(container, input, &stop))
				{
					error!("status updater failed: {err}");
				}
			}) {
			error!("failed to spawn status updater: {err}");
		}

		EVRInitError::VRInitError_None
	}

	fn Deactivate(&self) {
		self.status_updater_stop.store(true, Ordering::Relaxed);
		self.real.Deactivate()
	}

	fn EnterStandby(&self) {
		self.real.EnterStandby()
	}

	fn GetComponent(&self, pchComponentNameAndVersion: *const c_char) -> *mut c_void {
		let name = unsafe { CStr::from_ptr(pchComponentNameAndVersion) };
		info!("getting {name:?} hmd component");
		let real = self.real.GetComponent(pchComponentNameAndVersion);
		if name == unsafe { CStr::from_ptr(IVRDisplayComponent_Version) } {
			info!("replacing hmd display");
			self.display
		} else {
			real
		}
	}

	fn DebugRequest(
		&self,
		pchRequest: *const c_char,
		pchResponseBuffer: *mut c_char,
		unResponseBufferSize: u32,
	) {
		self.real
			.DebugRequest(pchRequest, pchResponseBuffer, unResponseBufferSize)
	}

	fn GetPose(&self) -> DriverPose_t {
		self.real.GetPose()
	}
}

struct PreparedHmd {
	display: *mut c_void,
	mode: Mode,
}
unsafe impl Send for PreparedHmd {}
unsafe impl Sync for PreparedHmd {}

static PREPARED_HMD: OnceLock<PreparedHmd> = OnceLock::new();

fn wrap_hmd(
	serial: *const c_char,
	real: *const VtableRef<ITrackedDeviceServerDriverVtable>,
) -> Result<*const VtableRef<ITrackedDeviceServerDriverVtable>> {
	let prepared = PREPARED_HMD
		.get()
		.ok_or(Error::Internal("hmd is not prepared"))?;
	let serial = unsafe { CStr::from_ptr(serial) }.to_string_lossy();
	SteamDevice::open(&serial)?;
	info!("wrapping hmd {serial}");
	let hmd = Box::leak(Box::new(WithVtables::new(HmdDriver {
		real: unsafe { VtableRef::from_raw(real) },
		display: prepared.display,
		mode: prepared.mode,
		status_updater_stop: Default::default(),
	})));
	Ok(HasVtable::<ITrackedDeviceServerDriverVtable>::get(hmd))
}

unsafe extern "C" fn tracked_device_added(
	host: &VtableRef<IVRServerDriverHostVtable>,
	pchDeviceSerialNumber: *const c_char,
	eDeviceClass: ETrackedDeviceClass,
	pDriver: *const VtableRef<ITrackedDeviceServerDriverVtable>,
) -> bool {
	let driver = if eDeviceClass == ETrackedDeviceClass::TrackedDeviceClass_HMD {
		wrap_hmd(pchDeviceSerialNumber, pDriver).unwrap_or_else(|err| {
			warn!("not wrapping hmd: {err}");
			pDriver
		})
	} else {
		pDriver
	};
	real_tracked_device_added()(host, pchDeviceSerialNumber, eDeviceClass, driver)
}

pub fn prepare_hmd() -> Result<()> {
	let vive = ViveDevice::open_first()?;

	let mode = {
		let res = HMD_RESOLUTION.get();
		let modes = vive.query_modes();
		let mode = *modes.iter().find(|m| m.id == res as u8).unwrap_or(
			modes
				.first()
				.expect("device has at least one mode if opened"),
		);
		HMD_RESOLUTION.set(mode.id as i32);

		vive.set_mode(mode.id)?;
		mode
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

	let vive_config = vive.read_config()?;

	let lens = LensLibrary::new()
		.and_then(|e| {
			e.set_config(vive_config.inhouse_lens_correction.clone())?;
			Ok(e)
		})
		.map(|v| Rc::new(v) as Rc<dyn LensClient>)
		.unwrap_or_else(|e| {
			fatal(format!("Lens distortion library is failed to load, HMD image most probaly will be distorted and unusable.\nError: {e}"));
			Rc::new(StubClient)
		});

	let display = Box::leak(Box::new(WithVtables::new(HmdDisplay { lens, mode })));
	let display = VtableRef::into_raw_mut(HasVtable::<IVRDisplayComponentVtable>::get_mut(display))
		as *mut c_void;
	PREPARED_HMD
		.set(PreparedHmd { display, mode })
		.map_err(|_| Error::Internal("hmd is already prepared"))?;

	hook_tracked_device_added(tracked_device_added)
}

fn fatal(s: String) {
	let zenity = var_os("STEAM_ZENITY").unwrap_or_else(|| OsString::from("zenity"));
	let mut cmd = Command::new(zenity);
	cmd.arg("--no-wrap").arg("--error").arg("--text").arg(&s);
	match cmd.spawn().and_then(|p| p.wait_with_output()) {
		Ok(v) => {
			info!(
				"zenity finished: {}\n{:?}\n{:?}",
				v.status, v.stdout, v.stderr
			)
		}
		Err(e) => {
			warn!("fatal error {s} remains unnoticed: {e}")
		}
	}
}
