#[macro_use]
extern crate log;
extern crate env_logger;

use atlas::event::Instance as EventInstance;
use std::process::exit;

fn main() {
	env_logger::init();

	info!("Initializing Sandbox...");

	let event = EventInstance::new()
		.title("Sandbox".to_owned())
		.resolution(1024, 768);

	match event.start() {
		Ok(_) => exit(0),
		Err(e) => {
			println!("Error: {:?}", e);
			exit(1);
		},
	}
}
