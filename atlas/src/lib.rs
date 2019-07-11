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

pub fn init((width, height): (u32, u32), title: String) {
	info!("Initializing Core...");

	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title(title)
		.with_inner_size((width, height).into())
		.build(&event_loop)
		.expect("Failed to unwrap window builder");

	event_loop.run(move |event, _, control_flow| match event {
		Event::EventsCleared => {
				// Application update code.
	
				// Queue a RedrawRequested event.
				window.request_redraw();
		},
		Event::WindowEvent {
				event: WindowEvent::RedrawRequested,
				..
		} => {
				// Redraw the application.
				//
				// It's preferrable to render in this event rather than in EventsCleared, since
				// rendering in here allows the program to gracefully handle redraws requested
				// by the OS.
		},
		Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
		} => *control_flow = ControlFlow::Exit,
		_ => *control_flow = ControlFlow::Wait,
	});
}
