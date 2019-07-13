#![crate_name = "atlas"]
#![warn(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;

#[cfg(windows)]
extern crate gfx_backend_dx12 as backend;
#[cfg(target_os = "macos")]
extern crate gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan as backend;
extern crate gfx_hal as hal;

pub mod error;
pub mod event;
