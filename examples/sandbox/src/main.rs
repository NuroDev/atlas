#[macro_use]
extern crate atlas;

use atlas::event::GameInstance;
use std::process::exit;

fn main() {
	env_logger::init();

	info!("Initializing Sandbox...");

	let game = GameInstance::new()
		.title("Sandbox".to_owned())
		.resolution(1024, 768);

	match game.start() {
		Ok(_) => exit(0),
		Err(e) => {
			println!("Error: {:?}", e);
			exit(1);
		},
	}
}
