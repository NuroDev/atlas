#[macro_use]
extern crate log;

#[cfg(windows)]
extern crate gfx_backend_dx12 as backend;
#[cfg(macos)]
extern crate gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan as backend;
extern crate gfx_hal as hal;

mod event;

use event::Instance as EventInstance;

pub fn init((width, height): (u32, u32), title: String) {
	info!("Initializing Atlas...");

	let instance = EventInstance::new().title(title).resolution(width, height);

	instance.start();
}
