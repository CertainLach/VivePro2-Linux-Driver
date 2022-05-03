use std::ffi::c_void;

use cppvtbl::VtableRef;
use openvr::{
	ECameraCompatibilityMode, ECameraVideoStreamFormat, EVRDistortionFunctionType,
	EVRTrackedCameraFrameType, HmdMatrix44_t, HmdVector2_t, ICameraVideoSinkCallbackVtable,
	IVRCameraComponent, IVRCameraComponentVtable,
};
use vive_hid::{ConfigCamera, DistortType};

struct HmdCamera {
	real: &'static VtableRef<IVRCameraComponentVtable>,
	cameras: Vec<ConfigCamera>,
}

impl IVRCameraComponent for HmdCamera {
	fn GetCameraFrameDimensions(
		&self,
		nVideoStreamFormat: ECameraVideoStreamFormat,
		pWidth: *mut u32,
		pHeight: *mut u32,
	) -> bool {
		self.real
			.GetCameraFrameDimensions(nVideoStreamFormat, pWidth, pHeight)
	}

	fn GetCameraFrameBufferingRequirements(
		&self,
		pDefaultFrameQueueSize: *mut i32,
		pFrameBufferDataSize: *mut u32,
	) -> bool {
		self.real
			.GetCameraFrameBufferingRequirements(pDefaultFrameQueueSize, pFrameBufferDataSize)
	}

	fn SetCameraFrameBuffering(
		&self,
		nFrameBufferCount: i32,
		ppFrameBuffers: *mut *mut c_void,
		nFrameBufferDataSize: u32,
	) -> bool {
		self.real
			.SetCameraFrameBuffering(nFrameBufferCount, ppFrameBuffers, nFrameBufferDataSize)
	}

	fn SetCameraVideoStreamFormat(&self, nVideoStreamFormat: ECameraVideoStreamFormat) -> bool {
		self.real.SetCameraVideoStreamFormat(nVideoStreamFormat)
	}

	fn GetCameraVideoStreamFormat(&self) -> ECameraVideoStreamFormat {
		self.real.GetCameraVideoStreamFormat()
	}

	fn StartVideoStream(&self) -> bool {
		self.real.StartVideoStream()
	}

	fn StopVideoStream(&self) {
		self.real.StopVideoStream()
	}

	fn IsVideoStreamActive(&self, pbPaused: *mut bool, pflElapsedTime: *mut f32) -> bool {
		self.real.IsVideoStreamActive(pbPaused, pflElapsedTime)
	}

	fn GetVideoStreamFrame(&self) -> *const openvr::CameraVideoStreamFrame_t {
		self.real.GetVideoStreamFrame()
	}

	fn ReleaseVideoStreamFrame(&self, pFrameImage: *const openvr::CameraVideoStreamFrame_t) {
		self.real.ReleaseVideoStreamFrame(pFrameImage)
	}

	fn SetAutoExposure(&self, bEnable: bool) -> bool {
		self.real.SetAutoExposure(bEnable)
	}

	fn PauseVideoStream(&self) -> bool {
		self.real.PauseVideoStream()
	}

	fn ResumeVideoStream(&self) -> bool {
		self.real.ResumeVideoStream()
	}

	fn GetCameraDistortion(
		&self,
		nCameraIndex: u32,
		flInputU: f32,
		flInputV: f32,
		pflOutputU: *mut f32,
		pflOutputV: *mut f32,
	) -> bool {
		self.real
			.GetCameraDistortion(nCameraIndex, flInputU, flInputV, pflOutputU, pflOutputV)
	}

	fn GetCameraProjection(
		&self,
		nCameraIndex: u32,
		eFrameType: EVRTrackedCameraFrameType,
		flZNear: f32,
		flZFar: f32,
		pProjection: *mut HmdMatrix44_t,
	) -> bool {
		self.real
			.GetCameraProjection(nCameraIndex, eFrameType, flZNear, flZFar, pProjection)
	}

	fn SetFrameRate(&self, nISPFrameRate: i32, nSensorFrameRate: i32) -> bool {
		self.real.SetFrameRate(nISPFrameRate, nSensorFrameRate)
	}

	fn SetCameraVideoSinkCallback(
		&self,
		pCameraVideoSinkCallback: *const VtableRef<ICameraVideoSinkCallbackVtable>,
	) -> bool {
		self.real
			.SetCameraVideoSinkCallback(pCameraVideoSinkCallback)
	}

	fn GetCameraCompatibilityMode(
		&self,
		pCameraCompatibilityMode: *mut ECameraCompatibilityMode,
	) -> bool {
		self.real
			.GetCameraCompatibilityMode(pCameraCompatibilityMode)
	}

	fn SetCameraCompatibilityMode(
		&self,
		nCameraCompatibilityMode: ECameraCompatibilityMode,
	) -> bool {
		self.real
			.SetCameraCompatibilityMode(nCameraCompatibilityMode)
	}

	fn GetCameraFrameBounds(
		&self,
		eFrameType: EVRTrackedCameraFrameType,
		pLeft: *mut u32,
		pTop: *mut u32,
		pWidth: *mut u32,
		pHeight: *mut u32,
	) -> bool {
		self.real
			.GetCameraFrameBounds(eFrameType, pLeft, pTop, pWidth, pHeight)
	}

	fn GetCameraIntrinsics(
		&self,
		nCameraIndex: u32,
		eFrameType: EVRTrackedCameraFrameType,
		pFocalLength: *mut HmdVector2_t,
		pCenter: *mut HmdVector2_t,
		peDistortionType: *mut EVRDistortionFunctionType,
		rCoefficients: *mut f64,
	) -> bool {
		let camera = if let Some(camera) = self.cameras.get(nCameraIndex as usize) {
			camera
		} else {
			tracing::warn!("unknown camera, fallback");
			return self.real.GetCameraIntrinsics(
				nCameraIndex,
				eFrameType,
				pFocalLength,
				pCenter,
				peDistortionType,
				rCoefficients,
			);
		};
		unsafe {
			*pFocalLength = HmdVector2_t {
				v: [camera.intrinsics.focal_x, camera.intrinsics.focal_y],
			};
			*pCenter = HmdVector2_t {
				v: [
					camera.intrinsics.distort.center_x,
					camera.intrinsics.distort.center_y,
				],
			};
			*peDistortionType = match camera.intrinsics.distort.r#type {
				DistortType::DistortFtheta => {
					EVRDistortionFunctionType::VRDistortionFunctionType_FTheta
				}
			};
			let slice = std::slice::from_raw_parts_mut(
				rCoefficients,
				camera.intrinsics.distort.coeffs.len(),
			);
			slice.copy_from_slice(&camera.intrinsics.distort.coeffs);
		};
		return true;
	}
}
