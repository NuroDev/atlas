#[macro_use]
extern crate log;

#[cfg(windows)]
extern crate gfx_backend_dx12 as backend;
#[cfg(macos)]
extern crate gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan as backend;
extern crate gfx_hal as hal;

use winit::{event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder};

pub fn init(title: String) {
	info!("Initializing Core...");

	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title(title)
		.build(&event_loop)
		.expect("Failed to unwrap window builder");

	event_loop.run(move |event, _, control_flow| match event {
		Event::WindowEvent {
			event: WindowEvent::CloseRequested,
			window_id,
		} if window_id == window.id() => *control_flow = ControlFlow::Exit,
		_ => *control_flow = ControlFlow::Wait,
	});
}
