use crate::graphics::types::{ColorFormat, Device, Factory, Resources};
use glutin::{
	dpi::LogicalSize,
	ContextBuilder,
	EventsLoop,
	PossiblyCurrent,
	Window,
	WindowBuilder,
	WindowedContext,
};
use imgui::Context;
use imgui_gfx_renderer::{Renderer, Shaders};

pub struct AtlasRenderer {
	/// TODO: Documentation
	pub renderer: Renderer<ColorFormat, Resources>,
	/// TODO: Documentation
	pub windowed_context: WindowedContext<PossiblyCurrent>,
	/// TODO: Documentation
	pub device: Device,
	/// TODO: Documentation
	pub factory: Factory,
	/// TODO: Documentation
	pub main_color: Option<gfx::handle::RenderTargetView<Resources, ColorFormat>>,
	/// TODO: Documentation
	pub main_depth: gfx::handle::DepthStencilView<Resources, gfx::format::DepthStencil>,
}

impl AtlasRenderer {
	/// TODO: Documentation
	pub fn new(
		imgui: &mut Context,
		builder: WindowBuilder,
		events_loop: &EventsLoop,
	) -> AtlasRenderer
	{
		{
			// Fix incorrect colors with sRGB framebuffer
			let style = imgui.style_mut();
			for col in 0..style.colors.len() {
				let color = style.colors[col];
				style.colors[col] = [
					color[0].powf(2.2),
					color[1].powf(2.2),
					color[2].powf(2.2),
					1.0 - (1.0 - color[3]).powf(2.2),
				];
			}
		}

		let context = ContextBuilder::new().with_vsync(true);

		let (windowed_context, device, mut factory, main_color, main_depth) =
			gfx_window_glutin::init(builder, context, &events_loop)
				.expect("Failed to initialize graphics");

		let shaders = {
			let version = device.get_info().shading_language;
			if version.is_embedded {
				if version.major >= 3 {
					Shaders::GlSlEs300
				} else {
					Shaders::GlSlEs100
				}
			} else if version.major >= 4 {
				Shaders::GlSl400
			} else if version.major >= 3 {
				if version.minor >= 2 {
					Shaders::GlSl150
				} else {
					Shaders::GlSl130
				}
			} else {
				Shaders::GlSl110
			}
		};

		let renderer =
			Renderer::init(imgui, &mut factory, shaders).expect("Failed to initialize renderer");

		AtlasRenderer {
			renderer,
			windowed_context,
			device,
			factory,
			main_color: Some(main_color),
			main_depth,
		}
	}

	/// TODO: Documentation
	pub fn swap_buffers(&mut self) { self.windowed_context.swap_buffers().unwrap(); }

	/// TODO: Documentation
	pub fn update_views(&mut self, _: LogicalSize) {
		if let Some(main_color) = self.main_color.as_mut() {
			gfx_window_glutin::update_views(
				&self.windowed_context,
				main_color,
				&mut self.main_depth,
			);
		}
	}

	/// TODO: Documentation
	pub fn window(&self) -> &Window { self.windowed_context.window() }
}
