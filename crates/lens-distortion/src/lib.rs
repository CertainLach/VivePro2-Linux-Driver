use std::{
	env::{self, vars_os},
	ffi::{OsStr, OsString},
	io,
	path::{Path, PathBuf},
	process::{Command, ExitStatus, Stdio},
	result,
};

use champagne::{override_import, FinishedPeImage, PeImage, VirtualPeb, VirtualTib};
use serde_json::Value;
use tracing::info_span;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("champagne: {0}")]
	Champagne(#[from] champagne::Error),
	#[error("fatal: {0}")]
	Fatal(&'static str),
}
type Result<T, E = Error> = result::Result<T, E>;

macro_rules! ensure {
	($($tt:tt)*) => {
		if !$($tt)* {
			return Err(Error::Fatal(stringify!($($tt)*)))
		}
	};
}

#[derive(Debug)]
pub struct DistortOutput {
	pub red: [f32; 2],
	pub green: [f32; 2],
	pub blue: [f32; 2],
}

#[derive(Debug)]
pub struct LeftRightTopBottom {
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Eye {
	Left = 0,
	Right = 1,
}

pub trait LensClient {
	fn project(&self, eye: Eye) -> Result<LeftRightTopBottom>;
	fn matrix_needs_inversion(&self) -> Result<bool>;
	fn distort(&self, eye: Eye, uv: [f32; 2]) -> Result<DistortOutput>;
	fn set_config(&self, config: Value) -> Result<()>;
}

pub struct LensLibrary {
	peb: VirtualPeb,
	m: LensLibraryMethods,
}
struct LensLibraryMethods {
	init: unsafe extern "win64" fn() -> u32,
	set_resolution: unsafe extern "win64" fn(width: u32, height: u32) -> u32,
	load_json_str: unsafe extern "win64" fn(str: *const u8, len: usize) -> u32,
	distort_uv: unsafe extern "win64" fn(
		eye: u32,
		color: u32,
		u: f32,
		v: f32,
		c1: *mut f32,
		c2: *mut f32,
	) -> u32,
	grow_for_undistort: unsafe extern "win64" fn(eye: u32, out: *mut [f32; 4]) -> u32,
	intrinsic: unsafe extern "win64" fn(eye: u32, out: *mut [f32; 8]) -> u32,
}
impl LensLibraryMethods {
	fn new(m: FinishedPeImage) -> Result<Self> {
		let init = unsafe { m.exported_fn("init")? };
		let set_resolution = unsafe { m.exported_fn("setResolution")? };
		let load_json_str = unsafe { m.exported_fn("loadJsonStr")? };
		let distort_uv = unsafe { m.exported_fn("distortUV")? };
		let grow_for_undistort = unsafe { m.exported_fn("getGrowForUndistort")? };
		let intrinsic = unsafe { m.exported_fn("getIntrinsic")? };

		Ok(Self {
			init,
			set_resolution,
			load_json_str,
			distort_uv,
			grow_for_undistort,
			intrinsic,
		})
	}
	fn init(&self) -> Result<()> {
		ensure!(unsafe { (self.init)() } == 0);
		Ok(())
	}
	fn set_resolution(&self, width: u32, height: u32) -> Result<()> {
		ensure!(unsafe { (self.set_resolution)(width, height) } == 0);
		Ok(())
	}
	fn load_json_str(&self, str: &str) -> Result<()> {
		ensure!(unsafe { (self.load_json_str)(str.as_ptr(), str.len()) } == 0);
		Ok(())
	}

	fn grow_for_undistort(&self, eye: Eye) -> Result<[f32; 4]> {
		let mut out = [0.0; 4];
		ensure!(unsafe { (self.grow_for_undistort)(eye as u32, &mut out) } == 0);
		Ok(out)
	}
	fn intrinsic(&self, eye: Eye) -> Result<[f32; 8]> {
		let mut out = [0.0; 8];
		ensure!(unsafe { (self.intrinsic)(eye as u32, &mut out) } == 0);
		Ok(out)
	}
	fn distort_uv(&self, eye: Eye, color: u32, uv: [f32; 2]) -> Result<[f32; 2]> {
		let mut a = 0.0;
		let mut b = 0.0;
		ensure!(unsafe { (self.distort_uv)(eye as u32, color, uv[0], uv[1], &mut a, &mut b) } == 0);
		Ok([a, b])
	}
}
impl LensLibrary {
	pub fn new() -> Result<Self> {
		let peb = VirtualPeb::new();
		let libs = find_libs()
			.unwrap_or_default()
			.to_str()
			.unwrap_or_default()
			.to_owned();

		let tib = VirtualTib::new(&peb);
		for lib in ["ucrtbase.dll", "vcruntime140.dll", "msvcp140.dll"] {
			let _ent_tib = tib.enter();
			let _span = info_span!("lib", lib = lib).entered();
			let mut m = PeImage::open(dbg!(format!("{libs}/{lib}")))?;
			m.resolve_imports(override_import, &peb)?;
			let m = m.finish()?;
			unsafe {
				m.init_static_tls()?;

				m.init_exceptions()?;
				m.call_ep_if_exists()?;
			}
		}
		{
			let _ent_tib = tib.enter();
			let _span = info_span!("opencv_world").entered();
			let mut m = PeImage::open(format!("{libs}/opencv_world346.dll"))?;
			m.resolve_imports(override_import, &peb)?;
			let m = m.finish()?;
			unsafe {
				m.init_static_tls()?;

				m.init_exceptions()?;
				m.call_ep_if_exists()?;
			}
		}
		let m = {
			let _ent_tib = tib.enter();
			let _span = info_span!("libdistort").entered();
			let mut m = PeImage::open(format!("{libs}/LibLensDistortion.dll"))?;
			m.resolve_imports(override_import, &peb)?;
			let m = m.finish()?;
			unsafe {
				m.init_static_tls()?;

				m.init_exceptions()?;
				m.call_ep_if_exists()?;
			}

			let _ent_tib = tib.enter();
			// unsafe { exercise_lens(&m)? };
			m
		};

		let m = LensLibraryMethods::new(m)?;

		let out = Self { peb, m };

		out.m.init()?;
		out.m.set_resolution(2448, 2448)?;

		Ok(out)
	}
}
impl LensClient for LensLibrary {
	fn project(&self, eye: Eye) -> Result<LeftRightTopBottom> {
		let mut g = self.m.grow_for_undistort(eye)?;
		for v in g.iter_mut() {
			*v += 1.0;
		}
		let i = self.m.intrinsic(eye)?;
		Ok(LeftRightTopBottom {
			left: (-1.0 - i[2]) * g[0] / i[0],
			right: (1.0 - i[2]) * g[1] / i[0],
			top: (1.0 - i[4 + 1]) * g[2] / i[4],
			bottom: (-1.0 - i[4 + 1]) * g[3] / i[4],
		})
	}

	fn matrix_needs_inversion(&self) -> Result<bool> {
		let v = self.project(Eye::Left)?;
		Ok(v.top > v.bottom)
	}

	fn distort(&self, eye: Eye, uv: [f32; 2]) -> Result<DistortOutput> {
		Ok(DistortOutput {
			red: self.m.distort_uv(eye, 2, uv)?,
			green: self.m.distort_uv(eye, 1, uv)?,
			blue: self.m.distort_uv(eye, 0, uv)?,
		})
	}

	fn set_config(&self, config: Value) -> Result<()> {
		let config_str =
			serde_json::to_string(&config).expect("serialization of values should not fail");
		self.m.load_json_str(&config_str)?;
		Ok(())
	}
}

pub struct StubClient;
impl LensClient for StubClient {
	fn project(&self, eye: Eye) -> Result<LeftRightTopBottom> {
		Ok(match eye {
			Eye::Left => LeftRightTopBottom {
				left: -1.667393,
				right: 0.821432,
				top: -1.116938,
				bottom: 1.122846,
			},
			Eye::Right => LeftRightTopBottom {
				left: -0.822435,
				right: 1.635135,
				top: -1.138235,
				bottom: 1.107449,
			},
		})
	}

	fn matrix_needs_inversion(&self) -> Result<bool> {
		Ok(true)
	}

	fn distort(&self, _eye: Eye, uv: [f32; 2]) -> Result<DistortOutput> {
		Ok(DistortOutput {
			red: uv,
			green: uv,
			blue: uv,
		})
	}

	fn set_config(&self, _config: Value) -> Result<()> {
		Ok(())
	}
}

pub fn find_libs() -> Option<PathBuf> {
	if let Some(path) = env::var_os("LENS_DISTORT_LIBS") {
		return Some(PathBuf::from(path));
	}
	let mut path = process_path::get_dylib_path()?;
	path.pop();
	path.push("lens-distort");
	Some(path)
}
