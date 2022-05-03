use std::result;

use crate::openvr::EVRInitError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("openvr error: {0:?}")]
	VR(EVRInitError),
	#[error("lens error: {0}")]
	Lens(#[from] lens_protocol::Error),
	#[error("lens client error: {0}")]
	LensClient(#[from] lens_client::Error),
	#[error("libloading: {0}")]
	LibLoading(#[from] libloading::Error),
	#[error("hid error: {0}")]
	Hid(#[from] vive_hid::Error),
	#[error("internal error: {0}")]
	Internal(&'static str),
}

impl From<EVRInitError> for Error {
	fn from(e: EVRInitError) -> Self {
		Self::VR(e)
	}
}

pub type Result<T, E = Error> = result::Result<T, E>;

#[macro_export]
macro_rules! try_vr {
	($($call:ident).+ ($($arg:expr),*)) => {{
		let mut error = crate::openvr::EVRInitError::VRInitError_None;
		let res = $($call).+($($arg,)* &mut error);

		if error != crate::openvr::EVRInitError::VRInitError_None {
			Err(crate::Error::VR(error))
		} else {
			Ok(res)
		}
	}};
}

#[macro_export]
macro_rules! vr_result {
	($result:ident, $res:expr, $err:expr) => {
		#[allow(unreachable_patterns)]
		match $res {
			Ok(v) => {
				if !$result.is_null() {
					unsafe { *$result = crate::openvr::EVRInitError::VRInitError_None };
				}
				v
			}
			Err(crate::Error::VR(vr)) => {
				if !$result.is_null() {
					unsafe { *$result = vr };
				}
				$err
			}
			Err(_) => {
				if !$result.is_null() {
					unsafe { *$result = crate::openvr::EVRInitError::VRInitError_Unknown };
				}
				$err
			}
		}
	};
}
