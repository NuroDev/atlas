use crate::{config::Config, error::Result};
use log::info;
use winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};

/// TODO: Documentation
pub trait EventHandler {
	/// TODO: Documentation
	fn load(&mut self) -> Result;
	/// TODO: Documentation
	fn update(&mut self) -> Result;
	/// TODO: Documentation
	fn draw(&mut self) -> Result;
}

pub fn run (config: Config) -> Result
{
	let mut event_loop = EventsLoop::new();
	let window = WindowBuilder::new()
		.with_title(&config.title)
		.with_dimensions((config.resolution.width, config.resolution.height).into())
		.build(&event_loop)
		.expect("Failed to unwrap window builder");

	info!("Starting event loop...");

	event_loop.run_forever(|event| match event {
		Event::WindowEvent {
			event: WindowEvent::CloseRequested,
			..
		} => ControlFlow::Break,
		_ => ControlFlow::Continue,
	});

	Ok(())
}
