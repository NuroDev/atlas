use atlas::{config::Config, event::Game, Result};
use log::{error, info};
use std::process::exit;

struct Events;

impl Game for Events {
	fn draw(&mut self) -> Result<()> {
		// Draw a new frame
		Ok(())
	}

	fn exit(&mut self) -> Result<()> {
		// Cleanup before closing the application
		Ok(())
	}

	fn load(&mut self) -> Result<()> {
		// Load the application & any assets
		Ok(())
	}

	fn update(&mut self) -> Result<()> {
        // Update every tick
        info!("Updating...");

		Ok(())
	}
}

fn main() {
	env_logger::init();

	info!("Initializing Events...");

	let config = Config::new().set_title("Events".to_owned());

	match atlas::event::run(config, Events) {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{:?}", e);
			exit(1);
		},
	}
}
