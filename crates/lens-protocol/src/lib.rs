#![feature(stdio_locked)]

use std::{
	io::{self, StdinLock, StdoutLock, Write},
	process::{Child, ChildStdin, ChildStdout},
	result,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("bincode error: {0}")]
	Bincode(#[from] bincode::Error),
	#[error("io error: {0}")]
	Io(#[from] io::Error),
	#[error("pipe failed")]
	MissingPipe,
	#[error("failed to ping server")]
	PingFailed,
}
type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DistortOutput {
	pub red: [f32; 2],
	pub green: [f32; 2],
	pub blue: [f32; 2],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeftRightTopBottom {
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum Eye {
	Left = 0,
	Right = 1,
}

mod json {
	use serde::{Deserialize, Deserializer, Serialize, Serializer};
	use serde_json::Value;

	pub fn serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let str = serde_json::to_string(&value).unwrap();
		str.serialize(serializer)
	}
	pub fn deserialize<'de, D>(deserializer: D) -> Result<Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		let str = String::deserialize(deserializer).unwrap();
		Ok(serde_json::from_str(&str).unwrap())
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
	Init(#[serde(with = "json")] Value),
	Ping(u32),
	Distort(Eye, [f32; 2]),
	ProjectionRaw(Eye),
	Exit,
}

pub struct Client {
	stdin: ChildStdin,
	stdout: ChildStdout,
	child: Child,
}
impl Client {
	pub fn open(mut child: Child, config: Value) -> Result<Self> {
		let mut res = Self {
			stdin: child.stdin.take().ok_or(Error::MissingPipe)?,
			stdout: child.stdout.take().ok_or(Error::MissingPipe)?,
			child,
		};

		if res.ping(0x12345678)? != 0x12345678 {
			return Err(Error::MissingPipe);
		}
		res.set_config(config)?;

		Ok(res)
	}
	pub fn request<R: DeserializeOwned>(&mut self, request: &Request) -> Result<R> {
		self.send(request)?;
		Ok(bincode::deserialize_from(&mut self.stdout)?)
	}
	pub fn send(&mut self, request: &Request) -> Result<()> {
		bincode::serialize_into(&self.stdin, request)?;
		self.stdin.flush()?;
		Ok(())
	}
	pub fn set_config(&mut self, config: Value) -> Result<()> {
		self.send(&Request::Init(config))?;
		Ok(())
	}
	pub fn ping(&mut self, v: u32) -> Result<u32> {
		Ok(self.request(&Request::Ping(v))?)
	}
	pub fn project(&mut self, eye: Eye) -> Result<LeftRightTopBottom> {
		Ok(self.request(&Request::ProjectionRaw(eye))?)
	}
	pub fn matrix_needs_inversion(&mut self) -> Result<bool> {
		let v = self.project(Eye::Left)?;
		Ok(v.top > v.bottom)
	}
	pub fn distort(&mut self, eye: Eye, uv: [f32; 2]) -> Result<DistortOutput> {
		self.request(&Request::Distort(eye, uv))
	}
	pub fn exit(&mut self) {
		// Flush may fail in case if exit succeeded
		let _ = self.send(&Request::Exit);
		self.child.wait().unwrap();
	}
}
impl Drop for Client {
	fn drop(&mut self) {
		self.exit()
	}
}

#[cfg(target_os = "windows")]
#[link(name = "msvcrt")]
extern "C" {
	fn _setmode(fd: i32, mode: i32) -> i32;
}

pub struct Server {
	stdin: StdinLock<'static>,
	stdout: StdoutLock<'static>,
	#[cfg(target_os = "windows")]
	modes: (i32, i32),
}
impl Server {
	pub fn listen() -> Self {
		#[cfg(target_os = "windows")]
		let modes = {
			let stdout = unsafe { _setmode(0, 0x8000) };
			let stdin = unsafe { _setmode(1, 0x8000) };
			assert!(
				stdout != -1 && stdin != -1,
				"binary mode should be accepted, and fds are correct"
			);
			(stdout, stdin)
		};

		let stdin = io::stdin_locked();
		let stdout = io::stdout_locked();

		Self {
			stdin,
			stdout,
			#[cfg(target_os = "windows")]
			modes,
		}
	}
	pub fn recv(&mut self) -> Result<Request> {
		Ok(bincode::deserialize_from(&mut self.stdin)?)
	}
	pub fn send(&mut self, v: &impl Serialize) -> Result<()> {
		bincode::serialize_into(&mut self.stdout, v)?;
		self.stdout.flush()?;
		Ok(())
	}
}
impl Drop for Server {
	fn drop(&mut self) {
		#[cfg(target_os = "windows")]
		{
			let stdout = unsafe { _setmode(0, self.modes.0) };
			let stdin = unsafe { _setmode(1, self.modes.1) };
			assert!(
				stdout != -1 && stdin != -1,
				"previous mode and fds should be correct"
			);
		}
	}
}
