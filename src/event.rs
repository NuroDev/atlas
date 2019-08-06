use crate::config::Config;
use crate::error::Result;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

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
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title(&config.title)
		.with_inner_size((config.width, config.height).into())
		.build(&event_loop)
		.expect("Failed to unwrap window builder");

	info!("Starting event loop...");

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
