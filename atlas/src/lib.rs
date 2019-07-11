#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;

#[cfg(windows)]
extern crate gfx_backend_dx12 as backend;
#[cfg(macos)]
extern crate gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan as backend;
extern crate gfx_hal as hal;

mod error;
mod event;

use event::Instance as EventInstance;

pub struct Atlas {
	pub name: String,
	pub event: EventInstance,
}

impl Default for Atlas {
	fn default() -> Atlas {
		Atlas {
			name: "Atlas".to_owned(),
			event: EventInstance::new()
		}
	}
}

impl Atlas {
	pub fn new () -> Self {
		Self::default()
	}

	pub fn name (mut self, name: String) -> Atlas {
		self.name = name;
		self
	}

	pub fn event (mut self, event: EventInstance) -> Atlas {
		self.event = event;
		self
	}

	pub fn start (&self) {
		info!("Initializing Atlas...");
		self.event.start()
	}
}
