use atlas::{Config, Event, Game, Result};

struct HelloWorld;

impl Game for HelloWorld {}

fn main() -> Result<()> {
	env_logger::init();

	let config: Config = Config::new().title("Hello World".to_owned());

	Event::new(config, Box::new(HelloWorld)).run()
}
