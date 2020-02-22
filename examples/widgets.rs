use atlas::{
	ui::{im_str, Condition, Ui, Window},
	Config,
	Event,
	Game,
	Result,
};
use log::{error, info};
use std::process::exit;

struct Widgets;

impl Game for Widgets {
	fn ui(&mut self, ui: &mut Ui) -> Result<()> {
		Window::new(im_str!("Hello widgets"))
			.size([400.0, 150.0], Condition::FirstUseEver)
			.build(ui, || {
				ui.text(im_str!("Hello world!"));
				ui.separator();

				ui.text(im_str!("FPS: {}", ui.io().framerate));

				let mouse_pos = ui.io().mouse_pos;
				ui.text(format!(
					"Mouse Position: ({:.1},{:.1})",
					mouse_pos[0], mouse_pos[1]
				));
			});

		ui.show_demo_window(&mut true);

		Ok(())
	}
}

fn main() {
	env_logger::init();

	info!("Initializing Widgets...");

	let config = Config::new().set_title("Widgets".to_string());
	let mut event: Event = Event::new(config, Box::new(Widgets));

	match event.run() {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{:?}", e);
			exit(1);
		},
	}
}
