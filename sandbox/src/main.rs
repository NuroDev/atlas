#[macro_use]
extern crate log;
extern crate env_logger;

use atlas::Atlas;

fn main() {
	env_logger::init();

	info!("Initializing Sandbox...");

	let sandbox = Atlas::new();
	
	sandbox.start();
}
