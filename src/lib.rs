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
//! ```sh
//! [dependencies]
//! atlas = { git = "https://github.com/nurodev/atlas" }
//! ```
//!
//! ## Example:
//! ```rust
//! use atlas::{config::Config, event::Game, Result};
//! use std::process::exit;
//!
//! struct MyGame;
//!
//! impl Game for MyGame {
//! 	fn draw(&mut self) -> Result<()> {
//! 		// Draw a new frame
//! 		Ok(())
//! 	}
//!
//! 	fn exit(&mut self) -> Result<()> {
//! 		// Cleanup before closing the application
//! 		Ok(())
//! 	}
//!
//! 	fn load(&mut self) -> Result<()> {
//! 		// Load the application & any assets
//! 		Ok(())
//! 	}
//!
//! 	fn update(&mut self) -> Result<()> {
//! 		// Update every tick
//! 		Ok(())
//! 	}
//! }
//!
//! fn main() {
//!     // Create a new application config
//!     let config = Config::new().set_title("My Game".to_owned());
//!
//!     // Run the application
//!     match atlas::event::run(config, MyGame) {
//!       Ok(_) => exit(0),
//!       Err(e) => panic!("{:?}", e),
//!     }
//! }

#![crate_name = "atlas"]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/NuroDev/atlas/master/book/src/assets/Logo.png"
)]
#![warn(dead_code)]

/// TODO: Documentation
pub type Result<T=()> = std::result::Result<T, error::AtlasError>;

// Core Modules
pub mod config;
pub mod error;
pub mod event;
pub mod graphics;

// Export Modules
pub extern crate nalgebra as math;
