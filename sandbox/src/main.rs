#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
	env_logger::init();

	info!("Initializing Sandbox...");

	atlas::init((1024, 768), "Sandbox".to_owned());
}
