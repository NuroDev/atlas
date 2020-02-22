use atlas::{Config, Event, Game, Result};
use log::{error, info};
use std::process::exit;

struct HelloWorld;

impl Game for HelloWorld {
	fn update(&mut self) -> Result<()> {
		// Update every tick
		info!("Tick");
		Ok(())
	}
}

fn main() {
	env_logger::init();

	info!("Initializing Hello World...");

	let config = Config::new().title("Hello World".to_owned());
	let mut event: Event = Event::new(config, Box::new(HelloWorld));

	match event.run() {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{:?}", e);
			exit(1);
		},
	}
}
