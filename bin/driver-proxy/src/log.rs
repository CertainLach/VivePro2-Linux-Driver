use std::{ffi::CString, io::Write};

use cppvtbl::VtableRef;
use once_cell::sync::OnceCell;

use crate::openvr::{IVRDriverLog, IVRDriverLogVtable};

static DRIVER_LOG: OnceCell<&'static VtableRef<IVRDriverLogVtable>> = OnceCell::new();

#[derive(Default)]
pub struct LogWriter(Vec<u8>);
impl LogWriter {
	fn flush_line(&mut self) {
		if let Some(driver) = DRIVER_LOG.get() {
			self.0.retain(|&x| x != 0);
			let v = CString::new(self.0.as_slice()).unwrap();
			driver.Log(v.as_ptr());
		} else {
			eprintln!("{}", String::from_utf8_lossy(self.0.as_slice()))
		}
		self.0.clear();
	}
}
impl Write for LogWriter {
	fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
		while let Some(pos) = buf.iter().position(|v| *v == b'\n') {
			self.0.extend_from_slice(&buf[..pos]);
			self.flush_line();
			buf = &buf[pos + 1..];
		}
		self.0.extend_from_slice(buf);
		Ok(buf.len())
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.flush_line();
		Ok(())
	}
}

pub fn try_init_driver_log(log: &'static VtableRef<IVRDriverLogVtable>) {
	let _ = DRIVER_LOG.set(log);
}
