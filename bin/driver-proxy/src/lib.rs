#![allow(non_snake_case, reason = "uses C++ names as is")]
#![allow(
	clippy::not_unsafe_ptr_arg_deref,
	reason = "safety is borked here because of how openvr is structured"
)]

#[macro_use]
extern crate openvr;

/// Wrappers for things, returned from original driver
mod driver;
pub use driver::{camera, hmd, server_tracked_provider};

/// Wrappers for things, passed from vrserver to original driver
mod server;
pub use server::{driver_context, driver_host};

#[macro_use]
mod error;
mod factory;
#[macro_use]
mod settings;
mod log;

pub use error::{Error, Result};
