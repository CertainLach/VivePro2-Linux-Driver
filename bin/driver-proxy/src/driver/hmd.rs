use std::{
	cell::RefCell,
	ffi::{c_void, CStr},
	os::raw::c_char,
	rc::Rc,
};

use crate::Result;
use cppvtbl::{impl_vtables, HasVtable, VtableRef, WithVtables};
use lens_protocol::{Client, Eye};
use tracing::{error, info, instrument};
use vive_hid::Mode;

use crate::openvr::{
	DistortionCoordinates_t, DriverPose_t, EVREye, EVRInitError, ITrackedDeviceServerDriver,
	ITrackedDeviceServerDriverVtable, IVRDisplayComponent, IVRDisplayComponentVtable,
	IVRDisplayComponent_Version,
};

fn map_eye(eye: EVREye) -> Eye {
	match eye {
		EVREye::Eye_Left => Eye::Left,
		EVREye::Eye_Right => Eye::Right,
	}
}

#[impl_vtables(IVRDisplayComponent)]
struct HmdDisplay {
	// steam: Rc<SteamDevice>,
	// vive: Rc<ViveDevice>,
	lens: Rc<RefCell<Client>>,
	real: &'static VtableRef<IVRDisplayComponentVtable>,
	mode: Mode,
}

impl IVRDisplayComponent for HmdDisplay {
	#[instrument(skip(self))]
	fn GetWindowBounds(&self, pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32) {
		// let err: Result<()> = try {
		let Mode { width, height, .. } = self.mode;
		unsafe {
			*pnX = 0;
			*pnY = 0;
			*pnWidth = width;
			*pnHeight = height;
		}
		// 	return;
		// };
		// error!("failed: {}", err.err().unwrap());
		// self.real.GetWindowBounds(pnX, pnY, pnWidth, pnHeight)
	}

	fn IsDisplayOnDesktop(&self) -> bool {
		self.real.IsDisplayOnDesktop()
	}

	fn IsDisplayRealDisplay(&self) -> bool {
		self.real.IsDisplayRealDisplay()
	}

	fn GetRecommendedRenderTargetSize(&self, pnWidth: *mut u32, pnHeight: *mut u32) {
		self.real.GetRecommendedRenderTargetSize(pnWidth, pnHeight)
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
		let err: Result<()> = try {
			let mut lens = self.lens.borrow_mut();
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
			return;
		};
		error!("failed: {}", err.err().unwrap());
		self.real
			.GetProjectionRaw(eEye, pfLeft, pfRight, pfTop, pfBottom)
	}

	#[instrument(skip(self))]
	fn ComputeDistortion(&self, eEye: EVREye, fU: f32, fV: f32) -> DistortionCoordinates_t {
		let err: Result<()> = try {
			let mut lens = self.lens.borrow_mut();
			let inverse = lens.matrix_needs_inversion()?;
			let result = lens.distort(map_eye(eEye), [fU, if inverse { 1.0 - fV } else { fV }])?;
			return DistortionCoordinates_t {
				rfRed: result.red,
				rfGreen: result.green,
				rfBlue: result.blue,
			};
		};
		error!("failed: {}", err.err().unwrap());
		self.real.ComputeDistortion(eEye, fU, fV)
	}
}

#[impl_vtables(ITrackedDeviceServerDriver)]
pub struct HmdDriver {
	// pub steam: Rc<SteamDevice>,
	pub lens: Rc<RefCell<Client>>,
	pub real: &'static VtableRef<ITrackedDeviceServerDriverVtable>,
	pub mode: Mode,
}

impl ITrackedDeviceServerDriver for HmdDriver {
	fn Activate(&self, unObjectId: u32) -> EVRInitError {
		self.real.Activate(unObjectId)
	}

	fn Deactivate(&self) {
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
			info!("faking display");
			let display = Box::leak(Box::new(WithVtables::new(HmdDisplay {
				// steam: self.steam.clone(),
				// vive: self.vive.clone(),
				lens: self.lens.clone(),
				real: unsafe { VtableRef::from_raw(real as *const _) },
				mode: self.mode,
			})));
			VtableRef::into_raw_mut(HasVtable::<IVRDisplayComponentVtable>::get_mut(display))
				as *mut _
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
