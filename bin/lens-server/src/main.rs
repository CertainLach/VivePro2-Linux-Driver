use std::{
	env::args,
	ffi::OsStr,
	process::exit,
	sync::atomic::{AtomicBool, Ordering},
};

use anyhow::{ensure, Context, Result};
use lens_protocol::{DistortOutput, Eye, LeftRightTopBottom, Request, Server};
use libloading::{Library, Symbol};
use serde_json::Value;
use tracing::info;

static CREATED: AtomicBool = AtomicBool::new(false);

struct LensLibraryMethods {
	init: Symbol<'static, unsafe extern "C" fn() -> u32>,
	set_resolution: Symbol<'static, unsafe extern "C" fn(width: u32, height: u32) -> u32>,
	load_json_str: Symbol<'static, unsafe extern "C" fn(str: *const u8, len: usize) -> u32>,
	distort_uv: Symbol<
		'static,
		unsafe extern "C" fn(
			eye: u32,
			color: u32,
			u: f32,
			v: f32,
			c1: &mut f32,
			c2: &mut f32,
		) -> u32,
	>,
	grow_for_undistort: Symbol<'static, unsafe extern "C" fn(eye: u32, out: &mut [f32; 4]) -> u32>,
	intrinsic: Symbol<'static, unsafe extern "C" fn(eye: u32, out: &mut [f32; 8]) -> u32>,
}
impl LensLibraryMethods {
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
struct LensLibrary {
	m: LensLibraryMethods,
	_marker: *const (),
}
impl LensLibrary {
	unsafe fn new(library: impl AsRef<OsStr>, resolution: (u32, u32)) -> Result<Self> {
		ensure!(
			!CREATED.swap(true, Ordering::Relaxed),
			"only single LensLibrary may exist per process"
		);
		let lib = Box::leak(Box::new(
			#[cfg(windows)]
			Library::from(
				libloading::os::windows::Library::load_with_flags(
					library,
					libloading::os::windows::LOAD_WITH_ALTERED_SEARCH_PATH,
				)
				.context("failed to load library")?,
			),
			#[cfg(not(windows))]
			Library::new(library).context("failed to load library")?,
		));
		let m = LensLibraryMethods {
			init: lib.get(b"init")?,
			set_resolution: lib.get(b"setResolution")?,
			load_json_str: lib.get(b"loadJsonStr")?,
			distort_uv: lib.get(b"distortUV")?,
			grow_for_undistort: lib.get(b"getGrowForUndistort")?,
			intrinsic: lib.get(b"getIntrinsic")?,
		};
		m.init()?;
		m.set_resolution(resolution.0, resolution.1)?;
		Ok(Self {
			m,
			_marker: std::ptr::null(),
		})
	}
	fn set_config(&self, config: Value) -> Result<()> {
		let config_str = serde_json::to_string(&config)?;
		self.m.load_json_str(&config_str)?;
		Ok(())
	}
	fn distort(&self, eye: Eye, uv: [f32; 2]) -> Result<DistortOutput> {
		Ok(DistortOutput {
			red: self.m.distort_uv(eye, 2, uv)?,
			green: self.m.distort_uv(eye, 1, uv)?,
			blue: self.m.distort_uv(eye, 0, uv)?,
		})
	}
	fn projection_raw(&self, eye: Eye) -> Result<LeftRightTopBottom> {
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
}

#[tracing::instrument(err)]
fn main() -> Result<()> {
	if args().skip(1).next() == Some("check".to_owned()) {
		exit(42);
	}

	tracing_subscriber::fmt()
		.without_time()
		// stdout is occupied for protocol, and stderr is available
		.with_writer(std::io::stderr)
		.init();
	info!("hello from lens server");
	let mut path = process_path::get_executable_path().context("failed to find executable path")?;
	path.pop();

	let mut dll_path = path.clone();
	dll_path.push("LibLensDistortion.dll");
	info!("dll path: {dll_path:?}");

	let mut server = Server::listen();

	let library = unsafe { LensLibrary::new(dll_path, (2448, 2448))? };

	loop {
		let req = server.recv().context("failed to read request")?;
		match req {
			Request::Init(config) => {
				info!("set config");
				library.set_config(config)?;
			}
			Request::Distort(eye, uv) => {
				server.send(&library.distort(eye, uv)?)?;
			}
			Request::ProjectionRaw(eye) => {
				server.send(&library.projection_raw(eye)?)?;
			}
			Request::Ping(v) => {
				server.send(&v)?;
			}
			Request::Exit => {
				info!("received exit signal");
				break;
			}
		}
	}
	Ok(())
}
