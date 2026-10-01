use std::{ffi::c_void, fs, io, os::raw::c_char, ptr::addr_of, sync::OnceLock};

use cppvtbl::VtableRef;

use crate::{
	driver_context::get_interface,
	openvr::{
		ETrackedDeviceClass, ITrackedDeviceServerDriverVtable, IVRServerDriverHostVtable,
		IVRServerDriverHost_Version,
	},
	Error, Result,
};

pub type TrackedDeviceAdded = unsafe extern "C" fn(
	&VtableRef<IVRServerDriverHostVtable>,
	*const c_char,
	ETrackedDeviceClass,
	*const VtableRef<ITrackedDeviceServerDriverVtable>,
) -> bool;

static REAL_TRACKED_DEVICE_ADDED: OnceLock<TrackedDeviceAdded> = OnceLock::new();

pub fn real_tracked_device_added() -> TrackedDeviceAdded {
	*REAL_TRACKED_DEVICE_ADDED
		.get()
		.expect("hook is called only after installation")
}

pub fn hook_tracked_device_added(hook: TrackedDeviceAdded) -> Result<()> {
	let host: &VtableRef<IVRServerDriverHostVtable> = get_interface(IVRServerDriverHost_Version)?;
	let vtable = unsafe {
		*(host as *const VtableRef<IVRServerDriverHostVtable> as *const *const IVRServerDriverHostVtable)
	};
	let slot = unsafe { addr_of!((*vtable).TrackedDeviceAdded) } as *mut TrackedDeviceAdded;
	let real = unsafe { slot.read() };
	if real as usize == hook as usize {
		return Ok(());
	}
	REAL_TRACKED_DEVICE_ADDED
		.set(real)
		.map_err(|_| Error::Internal("driver host is already hooked"))?;
	unsafe { write_protected(slot, hook) }
}

fn mapping_protection(addr: usize) -> Result<i32> {
	let maps = fs::read_to_string("/proc/self/maps")?;
	for line in maps.lines() {
		let mut parts = line.split_whitespace();
		let (Some(range), Some(perms)) = (parts.next(), parts.next()) else {
			continue;
		};
		let Some((start, end)) = range.split_once('-') else {
			continue;
		};
		let (Ok(start), Ok(end)) = (
			usize::from_str_radix(start, 16),
			usize::from_str_radix(end, 16),
		) else {
			continue;
		};
		if !(start..end).contains(&addr) {
			continue;
		}
		let perms = perms.as_bytes();
		let mut prot = libc::PROT_NONE;
		for (i, (flag, value)) in [
			(b'r', libc::PROT_READ),
			(b'w', libc::PROT_WRITE),
			(b'x', libc::PROT_EXEC),
		]
		.into_iter()
		.enumerate()
		{
			if perms.get(i) == Some(&flag) {
				prot |= value;
			}
		}
		return Ok(prot);
	}
	Err(Error::Internal("vtable is not mapped"))
}

unsafe fn write_protected<T>(slot: *mut T, value: T) -> Result<()> {
	let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
	let page = (slot as usize & !(page_size - 1)) as *mut c_void;
	let prot = mapping_protection(slot as usize)?;
	if libc::mprotect(page, page_size, prot | libc::PROT_WRITE) != 0 {
		return Err(io::Error::last_os_error().into());
	}
	slot.write(value);
	if libc::mprotect(page, page_size, prot) != 0 {
		return Err(io::Error::last_os_error().into());
	}
	Ok(())
}
