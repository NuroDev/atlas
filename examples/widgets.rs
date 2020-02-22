use atlas::{
	ui::{im_str, Condition, Ui, Window, WindowFlags},
	Config,
	Event,
	Game,
	Result,
};

const RESOLUTION: (f64, f64) = (1280.0, 720.0);
struct Widgets;

impl Game for Widgets {
	fn ui(&mut self, ui: &mut Ui) -> Result<()> {
		let offset: f32 = 10.0;
		let size: (f32, f32) = (150.0, 25.0);

		Window::new(im_str!("Metrics"))
			.size([size.0, size.1], Condition::Always)
			.flags(WindowFlags::NO_MOVE)
			.flags(WindowFlags::NO_DECORATION)
			.position(
				[(RESOLUTION.0 as f32) - size.0 - offset, offset],
				Condition::Always,
			)
			.build(ui, || {
				ui.text(im_str!("FPS: {}", ui.io().framerate));
			});

		ui.show_demo_window(&mut true);

		Ok(())
	}
}

fn main() -> Result<()> {
	env_logger::init();

	let config = Config::new()
		.title("Widgets".to_string())
		.resolution(RESOLUTION.0, RESOLUTION.1);

	Event::new(config, Box::new(Widgets)).run()
}
