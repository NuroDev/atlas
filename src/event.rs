use crate::{config::Config, error::Result};
use log::info;
use winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};

/// The interface between the engine and your game. Implement it, and pass your struct to `Window::main_loop`.
pub trait Game {
	/// Called every frame to draw/render content
	fn draw(&mut self) -> Result;
	/// Called on window close button click
	fn exit(&mut self) -> Result;
  /// Called at launch to initialize & load assets
	fn load(&mut self) -> Result;
	/// Called when every frame to update the game
	fn update(&mut self) -> Result;
}

pub fn run(config: Config) -> Result {
	let mut event_loop = EventsLoop::new();
	let _window = WindowBuilder::new()
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
