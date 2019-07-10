#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
	env_logger::init();

	info!("Initializing Sandbox...");

	atlas::init();
}
