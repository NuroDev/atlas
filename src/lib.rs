//! [![](https://raw.githubusercontent.com/NuroDev/atlas/master/book/src/assets/Banner.png)](https://github.com/nurodev/atlas)
//!
//! ## Features:
//! * Coming Soon
//!
//! ## Disclaimer:
//! Atlas is still a work-in-progress project and is not recommended for
//! production use.
//!
//! ## Installation:
//! You can install Atlas by adding Atlas as a dependency to you `Cargo.toml`
//! file.
//! ```toml
//! [dependencies]
//! atlas = { git = "https://github.com/nurodev/atlas" }
//! ```
//!
//! ## Example:
//! ```rust
//! use atlas::{
//! 	ui::{im_str, Condition, Ui, Window, WindowFlags},
//! 	Config,
//! 	Event,
//! 	Game,
//! 	Result,
//! };
//!
//! struct MyGame;
//!
//! fn main() -> Result<()> {
//! 	// Create a new application config
//! 		let config = Config::new().title("My Game".to_string());
//!
//! 	// Run the application
//! 		Event::new(config, Box::new(Widgets)).run()
//! }

#![crate_name = "atlas"]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/NuroDev/atlas/master/book/src/assets/Logo.png"
)]
#![warn(dead_code)]

/// TODO: Documentation
pub type Result<T=()> = std::result::Result<T, error::AtlasError>;

mod config;
mod error;
mod event;
mod graphics;

pub use config::Config;
pub use error::AtlasError as Error;
pub use event::AtlasEvent as Event;
pub use graphics::Game;

pub extern crate imgui as ui;
pub extern crate nalgebra as math;
