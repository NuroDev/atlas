use crate::error::AtlasError;
use winit::{event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder};

#[derive(Debug)]
pub struct Instance {
	pub title: String,
	pub width: u32,
	pub height: u32,
}

impl Default for Instance {
	fn default() -> Instance {
		Instance {
			title: "Atlas".to_owned(),
			width: 1024,
			height: 768,
		}
	}
}

impl Instance {
	pub fn new() -> Self { Self::default() }

	pub fn resolution(mut self, width: u32, height: u32) -> Instance {
		self.width = width;
		self.height = height;
		self
	}

	pub fn title(mut self, title: String) -> Instance {
		self.title = title;
		self
	}

	pub fn start(&self) -> Result<(), AtlasError> {
		let event_loop = EventLoop::new();
		let window = WindowBuilder::new()
			.with_title(&self.title)
			.with_inner_size((self.width, self.height).into())
			.build(&event_loop)
			.expect("Failed to unwrap window builder");

		event_loop.run(move |event, _, control_flow| match event {
			Event::EventsCleared => {
				window.request_redraw();
			},
			Event::WindowEvent {
				event: WindowEvent::RedrawRequested,
				..
			} => {},
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			_ => *control_flow = ControlFlow::Wait,
		});
	}
}
