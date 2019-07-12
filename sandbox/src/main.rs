#[macro_use]
extern crate log;
extern crate env_logger;

use atlas::Atlas;
use std::process::exit;

fn main() {
	env_logger::init();

	info!("Initializing Sandbox...");

	let sandbox = Atlas::new();

	match sandbox.start() {
		Ok(_) => exit(0),
		Err(e) => {
			println!("Error: {:?}", e);
			exit(1);
		},
	}
}
