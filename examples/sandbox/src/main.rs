#[macro_use]
extern crate atlas;

use atlas::config::{Config, WindowMode};
use atlas::error::Result;
use atlas::event::EventHandler;
use std::process::exit;

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

	let config = Config::new()
		.mode(WindowMode::Windowed)
		.title("Sandbox".to_owned())
		.resolution(1024, 768);

	match atlas::event::run(config) {
		Ok(_) => exit(0),
		Err(e) => {
			println!("Error: {:?}", e);
			exit(1);
		},
	}
}
