use std::{io::Read, result};

use flate2::read::ZlibDecoder;
use hidapi::{HidApi, HidDevice, HidError};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_json::Value;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("hid error: {0}")]
	Hid(#[from] HidError),
	#[error("device not found")]
	DeviceNotFound,
	#[error("device is not a vive device")]
	NotAVive,
	#[error("config size mismatch")]
	ConfigSizeMismatch,
	#[error("failed to read config")]
	ConfigReadFailed,
	#[error("protocol error: {0}")]
	ProtocolError(&'static str),
}

type Result<T, E = Error> = result::Result<T, E>;

static HIDAPI: OnceCell<HidApi> = OnceCell::new();
pub fn get_hidapi() -> Result<&'static HidApi> {
	HIDAPI.get_or_try_init(HidApi::new).map_err(From::from)
}

const STEAM_VID: u16 = 0x28de;
const STEAM_PID: u16 = 0x2300;

#[derive(Deserialize, Debug)]
pub struct ConfigDevice {
	pub eye_target_height_in_pixels: u32,
	pub eye_target_width_in_pixels: u32,
}
#[derive(Deserialize, Debug)]
pub enum DistortType {
	#[serde(rename = "DISTORT_FTHETA")]
	DistortFtheta,
}
#[derive(Deserialize, Debug)]
pub struct IntrinsicsDistort {
	pub center_x: f32,
	pub center_y: f32,
	pub coeffs: Vec<f64>,
	pub r#type: DistortType,
}
#[derive(Deserialize, Debug)]
pub struct ConfigCameraIntrinsics {
	pub center_x: f32,
	pub center_y: f32,
	pub distort: IntrinsicsDistort,
	pub focal_x: f32,
	pub focal_y: f32,
	pub width: u32,
	pub height: u32,
}
#[derive(Deserialize, Debug)]
pub struct ConfigCamera {
	pub name: String,
	pub intrinsics: ConfigCameraIntrinsics,
	pub extrinsics: Vec<u8>,
}
#[derive(Deserialize, Debug)]
pub struct SteamConfig {
	pub device: ConfigDevice,
	pub tracked_cameras: Vec<ConfigCamera>,
	pub direct_mode_edid_pid: u32,
	pub direct_mode_edid_vid: u32,
	pub seconds_from_photons_to_vblank: f64,
	pub seconds_from_vsync_to_photons: f64,
	/// SN of ViveDevice
	pub mb_serial_number: String,
}

pub struct SteamDevice(HidDevice);
impl SteamDevice {
	pub fn open_first() -> Result<Self> {
		let api = get_hidapi()?;
		let device = api.open(STEAM_VID, STEAM_PID)?;
		Ok(Self(device))
	}
	pub fn open(sn: &str) -> Result<Self> {
		let api = get_hidapi()?;
		let device = api
			.device_list()
			.find(|dev| dev.serial_number() == Some(sn))
			.ok_or(Error::DeviceNotFound)?;
		if device.vendor_id() != STEAM_VID || device.product_id() != STEAM_PID {
			return Err(Error::NotAVive);
		}
		let open = api.open_serial(device.vendor_id(), device.product_id(), sn)?;
		Ok(Self(open))
	}
	pub fn read_config(&self) -> Result<SteamConfig> {
		let mut report = [0u8; 64];
		report[0] = 16;
		let mut read_retries = 0;
		while self.0.get_feature_report(&mut report).is_err() {
			if read_retries > 5 {
				return Err(Error::ConfigReadFailed);
			}
			read_retries += 1;
		}
		read_retries = 0;
		let mut out = Vec::new();
		loop {
			report[0] = 17;
			if self.0.get_feature_report(&mut report).is_err() {
				if read_retries > 5 {
					return Err(Error::ConfigReadFailed);
				}
				read_retries += 1;
				continue;
			}
			read_retries = 0;
			if report[1] == 0 {
				break;
			}
			out.extend_from_slice(&report[2..2 + report[1] as usize])
		}
		let mut dec = ZlibDecoder::new(out.as_slice());
		let mut out = String::new();
		dec.read_to_string(&mut out)
			.map_err(|_| Error::ConfigReadFailed)?;

		serde_json::from_str(&out).map_err(|_| Error::ConfigReadFailed)
	}
}

const VIVE_VID: u16 = 0x0bb4;
const VIVE_PID: u16 = 0x0342;

#[derive(Deserialize, Debug)]
pub struct ViveConfig {
	pub device: ConfigDevice,
	pub direct_mode_edid_pid: u32,
	pub direct_mode_edid_vid: u32,
	pub seconds_from_photons_to_vblank: f64,
	pub seconds_from_vsync_to_photons: f64,
	/// Lets threat it as something opaque, anyway we directly feed this to lens-client
	pub inhouse_lens_correction: Value,
}

#[derive(Clone, Copy)]
pub struct Mode {
	pub id: u8,

	pub width: u32,
	pub height: u32,
	pub frame_rate: f32,
	pub extra_photon_vsync: f32,
}
impl Mode {
	const fn new(
		id: u8,
		width: u32,
		height: u32,
		frame_rate: f32,
		extra_photon_vsync: f32,
	) -> Self {
		Self {
			id,
			width,
			height,
			frame_rate,
			extra_photon_vsync,
		}
	}
}

const VIVE_PRO_2_MODES: [Mode; 6] = [
	Mode::new(0, 2448, 1224, 90.0, 0.0),
	Mode::new(1, 2448, 1224, 120.0, 0.0),
	Mode::new(2, 3264, 1632, 90.0, 0.00297),
	Mode::new(3, 3672, 1836, 90.0, 0.00332),
	Mode::new(4, 4896, 2448, 90.0, 0.0),
	Mode::new(5, 4896, 2448, 120.0, 0.0),
];

pub struct ViveDevice(HidDevice);
impl ViveDevice {
	pub fn open_first() -> Result<Self> {
		let api = get_hidapi()?;
		let device = api.open(VIVE_VID, VIVE_PID)?;
		Ok(Self(device))
	}
	pub fn open(sn: &str) -> Result<Self> {
		let api = get_hidapi()?;
		let device = api
			.device_list()
			.find(|dev| dev.serial_number() == Some(sn))
			.ok_or(Error::DeviceNotFound)?;
		if device.vendor_id() != VIVE_VID || device.product_id() != VIVE_PID {
			return Err(Error::NotAVive);
		}
		let open = api.open_serial(device.vendor_id(), device.product_id(), sn)?;
		Ok(Self(open))
	}
	fn write(&self, id: u8, data: &[u8]) -> Result<()> {
		let mut report = [0u8; 64];
		report[0] = id;
		report[1..1 + data.len()].copy_from_slice(data);
		self.0.write(&report)?;
		Ok(())
	}
	fn write_feature(&self, id: u8, sub_id: u16, data: &[u8]) -> Result<()> {
		let mut report = [0u8; 64];
		report[0] = id;
		report[1] = (sub_id & 0xff) as u8;
		report[2] = (sub_id >> 8) as u8;
		report[3] = data.len() as u8;
		report[4..][..data.len()].copy_from_slice(data);
		self.0.send_feature_report(&report)?;
		Ok(())
	}
	fn read(&self, id: u8, strip_prefix: &[u8], out: &mut [u8]) -> Result<usize> {
		let mut data = [0u8; 64];
		self.0.read(&mut data)?;
		if data[0] != id {
			error!("expected {id} but got {}\n{:02x?}", data[0], data);
			return Err(Error::ProtocolError("wrong report id"));
		}
		if &data[1..1 + strip_prefix.len()] != strip_prefix {
			error!(
				"expected {strip_prefix:x?}, got {:x?}",
				&data[1..1 + strip_prefix.len()]
			);
			return Err(Error::ProtocolError("wrong prefix"));
		}
		let size = data[1 + strip_prefix.len()] as usize;
		if size > 62 {
			return Err(Error::ProtocolError("wrong size"));
		}
		out[..size].copy_from_slice(&data[strip_prefix.len() + 2..strip_prefix.len() + 2 + size]);
		Ok(size)
	}
	pub fn read_devsn(&self) -> Result<String> {
		self.write(0x02, b"mfg-r-devsn")?;
		let mut out = [0u8; 62];
		let size = self.read(0x02, &[], &mut out)?;
		Ok(std::str::from_utf8(&out[..size])
			.map_err(|_| Error::ProtocolError("devsn is not a string"))?
			.to_string())
	}
	pub fn read_reg(&self, reg: &str) -> Result<String> {
		self.write(0x02, reg.as_bytes())?;
		let mut out = [0u8; 62];
		let size = self.read(0x02, &[], &mut out)?;
		Ok(std::str::from_utf8(&out[..size])
			.map_err(|_| Error::ProtocolError("result is not a string"))?
			.to_string())
	}
	pub fn read_config(&self) -> Result<ViveConfig> {
		let mut buf = [0u8; 62];
		// Request size
		let total_len = {
			self.write(0x01, &[0xea, 0xb1])?;
			let size = self.read(0x01, &[0xea, 0xb1], &mut buf)?;
			if size != 4 {
				return Err(Error::ProtocolError("config length has 4 bytes"));
			}
			let mut total_len = [0u8; 4];
			total_len.copy_from_slice(&buf[0..4]);
			u32::from_le_bytes(total_len) as usize
		};
		let mut read = 0;
		let mut out = Vec::<u8>::with_capacity(total_len);
		while read < total_len {
			let mut req = [0; 63];
			req[0] = 0xeb;
			req[1] = 0xb1;
			req[2] = 0x04;
			req[3..7].copy_from_slice(&u32::to_le_bytes(read as u32));

			self.write(0x01, &req)?;
			let size = self.read(0x01, &[0xeb, 0xb1], &mut buf)?;
			read += size;
			out.extend_from_slice(&buf[0..size]);
		}
		if read != total_len {
			return Err(Error::ProtocolError("config size mismatch"));
		}

		// First 128 bytes - something i can't decipher + sha256 hash (why?)
		let string = std::str::from_utf8(&out[128..])
			.map_err(|_| Error::ProtocolError("config is not utf-8"))?;

		serde_json::from_str(string).map_err(|_| Error::ConfigReadFailed)
	}
	/// Always returns at least one mode
	pub fn query_modes(&self) -> Vec<Mode> {
		VIVE_PRO_2_MODES.into_iter().collect()
	}
	pub fn set_mode(&self, resolution: u8) -> Result<(), Error> {
		self.write_feature(0x04, 0x2970, b"wireless,0")?;
		self.write_feature(0x04, 0x2970, format!("dtd,{}", resolution).as_bytes())?;
		self.write_feature(0x04, 0x2970, b"chipreset")?;
		// TODO: wait for reconnection
		Ok(())
	}
	pub fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
		self.write_feature(
			0x04,
			0x2970,
			format!("setbrightness,{}", brightness.min(130)).as_bytes(),
		)
	}
	pub fn toggle_noise_canceling(&self, enabled: bool) -> Result<(), Error> {
		const ENABLE: &[&[u8]] = &[
			b"codecreg=9c9,80".as_slice(),
			b"codecreg=9c8,a5",
			b"codecreg=9d0,a4",
			b"codecreg=1c008f,1",
			b"codecreg=1c0005,9",
			b"codecreg=1c0005,8000",
		];
		const DISABLE: &[&[u8]] = &[
			b"codecreg=9c9,8c".as_slice(),
			b"codecreg=9c8,a4",
			b"codecreg=9d0,0",
			b"codecreg=1c008f,0",
			b"codecreg=1c0005,9",
			b"codecreg=1c0005,8000",
		];
		// I have no idea what those values mean, this is straight
		// copy-pasta from what original vive console sends
		let lines: &'static [&'static [u8]] = if enabled { ENABLE } else { DISABLE };
		for line in lines {
			self.write_feature(0x04, 0x2971, line)?;
		}
		Ok(())
	}
}
