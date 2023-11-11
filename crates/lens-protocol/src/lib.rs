use std::{
	cell::RefCell,
	io::{self, Read, StdinLock, StdoutLock, Write},
	process::{Child, ChildStdin, ChildStdout},
	result,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("postcard error: {0}")]
	Postcard(#[from] postcard::Error),
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

pub trait LensClient {
	fn ping(&self, v: u32) -> Result<u32>;
	fn project(&self, eye: Eye) -> Result<LeftRightTopBottom>;
	fn matrix_needs_inversion(&self) -> Result<bool>;
	fn distort(&self, eye: Eye, uv: [f32; 2]) -> Result<DistortOutput>;
	fn set_config(&self, config: Value) -> Result<()>;
	fn exit(&self) -> Result<()>;
}

pub struct StubClient;
impl LensClient for StubClient {
	fn ping(&self, v: u32) -> Result<u32> {
		Ok(v)
	}

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

	fn exit(&self) -> Result<()> {
		Ok(())
	}
}

pub struct ServerClientInner {
	stdin: ChildStdin,
	stdout: ChildStdout,
	child: Child,
}
impl ServerClientInner {
	fn request<R: DeserializeOwned>(&mut self, request: &Request) -> Result<R> {
		self.send(request)?;
		let data = read_message(&mut self.stdout)?;
		Ok(postcard::from_bytes(&data)?)
	}
	pub fn send(&mut self, request: &Request) -> Result<()> {
		let data = postcard::to_stdvec(&request)?;
		write_message(&mut self.stdin, &data)?;
		self.stdin.flush()?;
		Ok(())
	}
}
pub struct ServerClient(RefCell<ServerClientInner>);
impl LensClient for ServerClient {
	fn ping(&self, v: u32) -> Result<u32> {
		self.0.borrow_mut().request(&Request::Ping(v))
	}
	fn project(&self, eye: Eye) -> Result<LeftRightTopBottom> {
		self.0.borrow_mut().request(&Request::ProjectionRaw(eye))
	}
	fn matrix_needs_inversion(&self) -> Result<bool> {
		let v = self.project(Eye::Left)?;
		Ok(v.top > v.bottom)
	}
	fn distort(&self, eye: Eye, uv: [f32; 2]) -> Result<DistortOutput> {
		self.0.borrow_mut().request(&Request::Distort(eye, uv))
	}

	fn set_config(&self, config: Value) -> Result<()> {
		self.0.borrow_mut().send(&Request::Init(config))?;
		Ok(())
	}

	fn exit(&self) -> Result<()> {
		// Flush may fail in case if exit succeeded
		let _ = self.0.borrow_mut().send(&Request::Exit);
		self.0.borrow_mut().child.wait().unwrap();
		Ok(())
	}
}
impl ServerClient {
	pub fn open(mut child: Child, config: Value) -> Result<Self> {
		let res = Self(RefCell::new(ServerClientInner {
			stdin: child.stdin.take().ok_or(Error::MissingPipe)?,
			stdout: child.stdout.take().ok_or(Error::MissingPipe)?,
			child,
		}));

		if res.ping(0x12345678)? != 0x12345678 {
			return Err(Error::MissingPipe);
		}
		res.set_config(config)?;

		Ok(res)
	}
	pub fn exit(&mut self) {}
}
impl Drop for ServerClient {
	fn drop(&mut self) {
		self.exit()
	}
}

#[cfg(target_os = "windows")]
#[link(name = "msvcrt")]
extern "C" {
	fn _setmode(fd: i32, mode: i32) -> i32;
}

pub fn read_message(read: &mut impl Read) -> Result<Vec<u8>> {
	let mut len_buf = [0; 4];
	read.read_exact(&mut len_buf)?;
	let len = u32::from_be_bytes(len_buf);
	// This protocol isn't talkative, its ok to allocate here.
	let mut data = vec![0; len as usize];
	read.read_exact(&mut data)?;
	Ok(data)
}
pub fn write_message(write: &mut impl Write, v: &[u8]) -> Result<()> {
	write.write_all(&u32::to_be_bytes(v.len() as u32))?;
	write.write_all(v)?;
	Ok(())
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

		let stdin = io::stdin().lock();
		let stdout = io::stdout().lock();

		Self {
			stdin,
			stdout,
			#[cfg(target_os = "windows")]
			modes,
		}
	}
	pub fn recv(&mut self) -> Result<Request> {
		let data = read_message(&mut self.stdin)?;
		Ok(postcard::from_bytes(&data)?)
	}
	pub fn send(&mut self, v: &impl Serialize) -> Result<()> {
		let data = postcard::to_stdvec(&v)?;
		write_message(&mut self.stdout, &data)?;
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
