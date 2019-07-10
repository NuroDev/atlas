#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init();

    info!("Initializing Sandbox...");

    core::log();
    physics::log();
    renderer::log();
}
