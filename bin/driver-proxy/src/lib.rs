#![feature(never_type, try_blocks)]
#![allow(non_snake_case)]

#[macro_use]
extern crate openvr;

mod driver_context;
mod driver_host;
#[macro_use]
mod error;
mod camera;
mod factory;
mod hmd;
mod server_tracked_provider;
#[macro_use]
mod settings;
mod log;

pub use error::{Error, Result};
