use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use libloading::os::windows::{Library, Symbol};
use std::ffi::OsStr;
use std::io::{self, Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
struct DistortOutput {
    red: [f32; 2],
    green: [f32; 2],
    blue: [f32; 2],
}
impl Output for DistortOutput {
    fn write<W: Write>(self, w: &mut W) {
        w.write_f32::<LE>(self.red[0]).unwrap();
        w.write_f32::<LE>(self.red[1]).unwrap();
        w.write_f32::<LE>(self.green[0]).unwrap();
        w.write_f32::<LE>(self.green[1]).unwrap();
        w.write_f32::<LE>(self.blue[0]).unwrap();
        w.write_f32::<LE>(self.blue[1]).unwrap();
    }
}
#[derive(Debug)]
struct LeftRightTopBottom {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}
impl Output for LeftRightTopBottom {
    fn write<W: Write>(self, w: &mut W) {
        w.write_f32::<LE>(self.left).unwrap();
        w.write_f32::<LE>(self.right).unwrap();
        w.write_f32::<LE>(self.top).unwrap();
        w.write_f32::<LE>(self.bottom).unwrap();
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
enum Eye {
    Left = 0,
    Right = 1,
}
impl Eye {
    fn read<R: Read>(r: &mut R) -> Self {
        match r.read_u8().unwrap() {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!(),
        }
    }
}

static CREATED: AtomicBool = AtomicBool::new(false);

struct LensLibraryMethods {
    init: Symbol<unsafe extern "C" fn() -> u32>,
    set_resolution: Symbol<unsafe extern "C" fn(width: u32, height: u32) -> u32>,
    load_json_str: Symbol<unsafe extern "C" fn(str: *const u8, len: usize) -> u32>,
    distort_uv: Symbol<
        unsafe extern "C" fn(
            eye: u32,
            color: u32,
            u: f32,
            v: f32,
            c1: &mut f32,
            c2: &mut f32,
        ) -> u32,
    >,
    grow_for_undistort: Symbol<unsafe extern "C" fn(eye: u32, out: &mut [f32; 4]) -> u32>,
    intrinsic: Symbol<unsafe extern "C" fn(eye: u32, out: &mut [f32; 8]) -> u32>,
}
impl LensLibraryMethods {
    fn init(&self) {
        assert_eq!(unsafe { (self.init)() }, 0);
    }
    fn set_resolution(&self, width: u32, height: u32) {
        assert_eq!(unsafe { (self.set_resolution)(width, height) }, 0);
    }
    fn load_json_str(&self, str: &str) {
        assert_eq!(unsafe { (self.load_json_str)(str.as_ptr(), str.len()) }, 0)
    }

    fn grow_for_undistort(&self, eye: Eye) -> [f32; 4] {
        let mut out = [0.0; 4];
        assert_eq!(
            unsafe { (self.grow_for_undistort)(eye as u32, &mut out) },
            0
        );
        out
    }
    fn intrinsic(&self, eye: Eye) -> [f32; 8] {
        let mut out = [0.0; 8];
        assert_eq!(unsafe { (self.intrinsic)(eye as u32, &mut out) }, 0);
        out
    }
    fn distort_uv(&self, eye: Eye, color: u32, uv: [f32; 2]) -> [f32; 2] {
        let mut a = 0.0;
        let mut b = 0.0;
        assert_eq!(
            unsafe { (self.distort_uv)(eye as u32, color, uv[0], uv[1], &mut a, &mut b) },
            0
        );
        [a, b]
    }
}
struct LensLibrary {
    m: LensLibraryMethods,
    _marker: *const (),
}
impl LensLibrary {
    unsafe fn new(library: impl AsRef<OsStr>, resolution: (u32, u32), json: &str) -> Self {
        assert!(
            !CREATED.swap(true, Ordering::Relaxed),
            "only single LensLibrary may exist per process"
        );
        let lib = Box::leak(Box::new(
            Library::load_with_flags(library, 0x00000008).expect("failed to load library"),
        ));
        let m = LensLibraryMethods {
            init: lib.get(b"init").unwrap(),
            set_resolution: lib.get(b"setResolution").unwrap(),
            load_json_str: lib.get(b"loadJsonStr").unwrap(),
            distort_uv: lib.get(b"distortUV").unwrap(),
            grow_for_undistort: lib.get(b"getGrowForUndistort").unwrap(),
            intrinsic: lib.get(b"getIntrinsic").unwrap(),
        };
        m.init();
        m.set_resolution(resolution.0, resolution.1);
        m.load_json_str(json);
        Self {
            m,
            _marker: std::ptr::null(),
        }
    }
    fn distort(&self, eye: Eye, uv: [f32; 2]) -> DistortOutput {
        DistortOutput {
            red: self.m.distort_uv(eye, 2, uv),
            green: self.m.distort_uv(eye, 1, uv),
            blue: self.m.distort_uv(eye, 0, uv),
        }
    }
    fn projection_raw(&self, eye: Eye) -> LeftRightTopBottom {
        let mut g = self.m.grow_for_undistort(eye);
        for v in g.iter_mut() {
            *v += 1.0;
        }
        let i = self.m.intrinsic(eye);
        LeftRightTopBottom {
            left: (-1.0 - i[2]) * g[0] / i[0],
            right: (1.0 - i[2]) * g[1] / i[0],
            top: (1.0 - i[4 + 1]) * g[2] / i[4],
            bottom: (-1.0 - i[4 + 1]) * g[3] / i[4],
        }
    }
}

enum Input {
    Distort(Eye, [f32; 2]),
    ProjectionRaw(Eye),
}
impl Input {
    fn read<R: Read>(r: &mut R) -> Self {
        match r.read_u8().unwrap() {
            0 => Self::Distort(
                Eye::read(r),
                [r.read_f32::<LE>().unwrap(), r.read_f32::<LE>().unwrap()],
            ),
            1 => Self::ProjectionRaw(Eye::read(r)),
            _ => panic!(),
        }
    }
}

trait Output {
    fn write<W: Write>(self, w: &mut W);
}

// center pos - to int
// scale ratio - to int
fn main() {
    let mut exedir = std::env::current_exe().unwrap();
    exedir.pop();

    let mut config_path = exedir.clone();
    config_path.push("config.json");
    let config = std::fs::read_to_string(config_path).expect("failed to read config");

    let mut distortion_path = exedir.clone();
    distortion_path.push("LibLensDistortion.dll");
    let lib = unsafe { LensLibrary::new(distortion_path, (1632, 1632), &config) };
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    loop {
        match Input::read(&mut stdin) {
            Input::Distort(eye, uv) => lib.distort(eye, uv).write(&mut stdout),
            Input::ProjectionRaw(eye) => lib.projection_raw(eye).write(&mut stdout),
        };
        stdout.flush().unwrap();
    }
}
