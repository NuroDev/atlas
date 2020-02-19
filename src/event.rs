use crate::{
	config::Config,
	graphics::{Game, Renderer},
	Result,
};
use gfx::Encoder;
use glutin::{dpi::LogicalSize, Event, EventsLoop, WindowBuilder, WindowEvent};
use imgui::Context;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use log::info;
use std::time::Instant;

pub struct AtlasEvent {
	/// TODO: Documentation
	pub config: Config,
	/// TODO: Documentation
	pub game: Box<dyn Game>,
}

impl AtlasEvent {
	pub fn new(config: Config, game: Box<dyn Game>) -> AtlasEvent {
		AtlasEvent {
			config,
			game,
		}
	}

	pub fn run(&mut self) -> Result<()> {
		info!("Starting event loop...");

		// Create primary app event loop
		let mut events_loop = EventsLoop::new();

		// Create app window builder
		let builder = WindowBuilder::new()
			.with_title(&self.config.title)
			.with_dimensions(LogicalSize::new(
				self.config.resolution.width,
				self.config.resolution.height,
			));

		// Create device context
		let mut context = Context::create();
		context.set_ini_filename(None);

		// Initialize window platform using winit
		let mut platform = WinitPlatform::init(&mut context);

		// Get current DPI factor from winit
		let hidpi_factor = platform.hidpi_factor();

		// Set gloval font size based on dpi factor
		context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

		// Create renderer instance using previously created context, window builder
		// and primary event loop
		let mut renderer_sys = Renderer::new(&mut context, builder, &events_loop);
		platform.attach_window(context.io_mut(), renderer_sys.window(), HiDpiMode::Rounded);

		let mut encoder: Encoder<_, _> = renderer_sys.factory.create_command_buffer().into();

		let mut last_frame = Instant::now();
		let mut is_running = true;

		while is_running {
			// Window event handler
			events_loop.poll_events(|event| {
				platform.handle_event(context.io_mut(), renderer_sys.window(), &event);

				if let Event::WindowEvent {
					event, ..
				} = event
				{
					match event {
						WindowEvent::Resized(size) => renderer_sys.update_views(size),
						WindowEvent::CloseRequested => is_running = false,
						_ => (),
					}
				}
			});

			let io = context.io_mut();
			platform
				.prepare_frame(io, renderer_sys.window())
				.expect("Failed to start frame");
			last_frame = io.update_delta_time(last_frame);
			let mut ui = context.frame();

			self.game.update()?;
			self.game.draw()?;
			self.game.ui(&mut ui)?;

			if let Some(main_color) = renderer_sys.main_color.as_mut() {
				encoder.clear(main_color, [0.0, 0.0, 0.0, 0.0]);
			}
			platform.prepare_render(&ui, renderer_sys.window());
			let draw_data = ui.render();
			if let Some(main_color) = renderer_sys.main_color.as_mut() {
				renderer_sys
					.renderer
					.render(
						&mut renderer_sys.factory,
						&mut encoder,
						main_color,
						draw_data,
					)
					.expect("Rendering failed");
			}

			encoder.flush(&mut renderer_sys.device);
			renderer_sys.swap_buffers();

			// TODO
			// renderer_sys.device.cleanup();
		}

		Ok(())
	}
}
