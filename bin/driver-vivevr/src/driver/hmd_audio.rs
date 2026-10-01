use std::{
	ffi::CStr,
	fs::{self, File, OpenOptions},
	io,
	mem::{size_of, zeroed},
	os::fd::AsRawFd,
	time::{Duration, Instant},
};

use tracing::{info, warn};
use vive_hid::HmdStatus;

use crate::{Error, Result};

const VIVE_AUDIO_USB_ID: &str = "0bb4:030b";
const SPEAKER_VOLUME: &CStr = c"Speaker Playback Volume";
const MIC_SWITCH: &CStr = c"Mic Capture Switch";
const REPEAT_DELAY: Duration = Duration::from_millis(500);
const REPEAT_INTERVAL: Duration = Duration::from_millis(150);

const SNDRV_CTL_ELEM_IFACE_MIXER: i32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
struct ElemId {
	numid: u32,
	iface: i32,
	device: u32,
	subdevice: u32,
	name: [u8; 44],
	index: u32,
}

#[repr(C)]
struct ElemInfo {
	id: ElemId,
	elem_type: i32,
	access: u32,
	count: u32,
	owner: i32,
	min: i64,
	max: i64,
	step: i64,
	value_reserved: [u8; 104],
	reserved: [u8; 64],
}

#[repr(C)]
struct ElemValue {
	id: ElemId,
	indirect: u32,
	integer: [i64; 128],
	reserved: [u8; 128],
}

const _: () = assert!(size_of::<ElemId>() == 64);
const _: () = assert!(size_of::<ElemInfo>() == 272);
const _: () = assert!(size_of::<ElemValue>() == 1224);

const fn iowr<T>(nr: u8) -> u32 {
	(3 << 30) | ((size_of::<T>() as u32) << 16) | ((b'U' as u32) << 8) | nr as u32
}
const SNDRV_CTL_IOCTL_ELEM_INFO: u32 = iowr::<ElemInfo>(0x11);
const SNDRV_CTL_IOCTL_ELEM_READ: u32 = iowr::<ElemValue>(0x12);
const SNDRV_CTL_IOCTL_ELEM_WRITE: u32 = iowr::<ElemValue>(0x13);

fn mixer_id(name: &CStr) -> ElemId {
	let mut id: ElemId = unsafe { zeroed() };
	id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
	let name = name.to_bytes();
	id.name[..name.len()].copy_from_slice(name);
	id
}

struct HmdAudio {
	control: File,
}

impl HmdAudio {
	fn open() -> Result<Self> {
		for entry in fs::read_dir("/proc/asound")? {
			let path = entry?.path();
			let Some(card) = path
				.file_name()
				.and_then(|n| n.to_str())
				.and_then(|n| n.strip_prefix("card"))
				.and_then(|n| n.parse::<u32>().ok())
			else {
				continue;
			};
			let Ok(usb_id) = fs::read_to_string(path.join("usbid")) else {
				continue;
			};
			if usb_id.trim() == VIVE_AUDIO_USB_ID {
				let control = OpenOptions::new()
					.read(true)
					.write(true)
					.open(format!("/dev/snd/controlC{card}"))?;
				return Ok(Self { control });
			}
		}
		Err(Error::Internal("hmd audio card not found"))
	}

	unsafe fn ioctl<T>(&self, request: u32, arg: &mut T) -> Result<()> {
		if libc::ioctl(self.control.as_raw_fd(), request as _, arg as *mut T) < 0 {
			return Err(io::Error::last_os_error().into());
		}
		Ok(())
	}

	fn info(&self, name: &CStr) -> Result<ElemInfo> {
		let mut info: ElemInfo = unsafe { zeroed() };
		info.id = mixer_id(name);
		unsafe { self.ioctl(SNDRV_CTL_IOCTL_ELEM_INFO, &mut info)? };
		Ok(info)
	}

	fn read(&self, name: &CStr) -> Result<ElemValue> {
		let mut value: ElemValue = unsafe { zeroed() };
		value.id = mixer_id(name);
		unsafe { self.ioctl(SNDRV_CTL_IOCTL_ELEM_READ, &mut value)? };
		Ok(value)
	}

	fn write(&self, value: &mut ElemValue) -> Result<()> {
		unsafe { self.ioctl(SNDRV_CTL_IOCTL_ELEM_WRITE, value) }
	}

	fn step_volume(&self, delta: i64) -> Result<i64> {
		let info = self.info(SPEAKER_VOLUME)?;
		let mut value = self.read(SPEAKER_VOLUME)?;
		let count = (info.count as usize).min(value.integer.len());
		for channel in &mut value.integer[..count] {
			*channel = (*channel + delta).clamp(info.min, info.max);
		}
		self.write(&mut value)?;
		Ok(value.integer[0])
	}

	fn toggle_mic(&self) -> Result<bool> {
		let info = self.info(MIC_SWITCH)?;
		let mut value = self.read(MIC_SWITCH)?;
		let enabled = value.integer[0] == 0;
		let count = (info.count as usize).min(value.integer.len());
		value.integer[..count].fill(enabled as i64);
		self.write(&mut value)?;
		Ok(!enabled)
	}
}

#[derive(Default)]
pub struct HmdAudioKeys {
	audio: Option<HmdAudio>,
	repeat_at: Option<Instant>,

	volume_up: bool,
	volume_down: bool,
	mute_mic: bool,
}

impl HmdAudioKeys {
	pub fn update(&mut self, buttons: HmdStatus) {
		let now = Instant::now();
		let delta = buttons.volume_up as i64 - buttons.volume_down as i64;
		let changed =
			buttons.volume_up != self.volume_up || buttons.volume_down != self.volume_down;
		if delta == 0 {
			self.repeat_at = None;
		} else if changed {
			self.step_volume(delta);
			self.repeat_at = Some(now + REPEAT_DELAY);
		} else if self.repeat_at.is_some_and(|at| now >= at) {
			self.step_volume(delta);
			self.repeat_at = Some(now + REPEAT_INTERVAL);
		}
		self.volume_up = buttons.volume_up;
		self.volume_down = buttons.volume_down;
		if buttons.mute_mic && !self.mute_mic {
			self.toggle_mic();
		}
		self.mute_mic = buttons.mute_mic;
	}

	fn with_audio<T>(&mut self, f: impl FnOnce(&HmdAudio) -> Result<T>) -> Option<T> {
		if self.audio.is_none() {
			match HmdAudio::open() {
				Ok(audio) => self.audio = Some(audio),
				Err(err) => {
					warn!("hmd audio is unavailable: {err}");
					return None;
				}
			}
		}
		let result = f(self.audio.as_ref()?);
		match result {
			Ok(v) => Some(v),
			Err(err) => {
				warn!("hmd audio control failed: {err}");
				self.audio = None;
				None
			}
		}
	}

	fn step_volume(&mut self, delta: i64) {
		if let Some(volume) = self.with_audio(|audio| audio.step_volume(delta)) {
			info!("hmd volume: {volume}");
		}
	}

	fn toggle_mic(&mut self) {
		if let Some(muted) = self.with_audio(|audio| audio.toggle_mic()) {
			info!("hmd mic muted: {muted}");
		}
	}
}
