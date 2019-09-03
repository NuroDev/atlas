use atlas::{config::Config,
            error::Result,
            event::EventHandler};
use log::{error, info};
use std::process::exit;

#[allow(dead_code)]
struct Sandbox {}

impl EventHandler for Sandbox {
	fn load(&mut self) -> Result<()> {
		// Load the application & any assets
		Ok(())
	}

	fn draw(&mut self) -> Result<()> {
		// Draw a new frame
		Ok(())
	}

	fn update(&mut self) -> Result<()> {
		// Update every tick
		Ok(())
	}
}

fn main() {
	env_logger::init();

	info!("Initializing Sandbox...");

	let config = Config::new().title("Sandbox".to_owned());

	match atlas::event::run(config) {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{:?}", e);
			exit(1);
		},
	}
}
