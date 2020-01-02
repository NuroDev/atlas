use atlas::{config::Config, event::Game, Result};
use log::{error, info};
use std::process::exit;

struct Triangle;

impl Game for Triangle {
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
		Ok(())
	}
}

fn main() {
	env_logger::init();

	info!("Initializing Triangle...");

	let config = Config::new().set_title("Triangle".to_owned());

	match atlas::event::run(config, Triangle) {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{:?}", e);
			exit(1);
		},
	}
}
