#[macro_use]
extern crate log;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn init() {
  info!("Initializing Core...");

  physics::init();
  renderer::init();

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Atlas")
    .build(&event_loop)
    .expect("Failed to unwrap window builder");

  event_loop.run(move |event, _, control_flow| {
      match event {
          Event::WindowEvent {
              event: WindowEvent::CloseRequested,
              window_id,
          } if window_id == window.id() => *control_flow = ControlFlow::Exit,
          _ => *control_flow = ControlFlow::Wait,
      }
  });
}
