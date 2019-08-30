/*!
[![](https://raw.githubusercontent.com/NuroDev/atlas/master/assets/Banner.png)](https://github.com/nurodev/atlas)

## Features: 
* Coming Soon

# What is Atlas?
Atlas is a proof of concept lightweight 3D game engine built entirely in Rust.

## Why?
At the time of writing I am about to enter my final year at university studying computer games programming.
As part of my final year, I need to create a final year project in the form of a case study.
Since the project can be on anything we so choose, one of the ideas that I came up with was to try and build a game engine in Rust.

C++ is the backbone of the games industry and will most likely continue to do so for a very long time.
But before returning for my final year I had been working at a creative studio that was already using Rust and I rapidly began to fall in love with its safety and performance that was comporable to C++.

So I began to some research into the current state of Rust and using it for Games development and was pleasantly surprised to find a rapidly growing community of people looking to create games/game engines using Rust.
After some initial trials and testing with crates such as Amethyst, Piston, Kiss3d, ggez and many others, I decided it was time to go all in and begin creating a cross platform 3D engine built entirely in Rust.

## Disclaimer: 
Atlas is still a work-in-progress project and is not recommended for production use.

## Installation:
You can install Atlas by adding Atlas as a dependency to you `Cargo.toml` file.
```sh
[dependencies]
atlas = { git = "https://github.com/nurodev/atlas" }
```

## Example:
```rust
use atlas::config::Config;
use atlas::error::Result;
use atlas::event::EventHandler;
use std::process::exit;

#[allow(dead_code)]
struct MyGame {
  // Game assets & state
}

impl EventHandler for MyGame {
  fn load(&mut self) -> Result {
    // Load the application & any assets
    Ok(())
  }

  fn draw(&mut self) -> Result {
    // Draw a new frame
    Ok(())
  }
  
  fn update (&mut self) -> Result {
    // Update every tick
    Ok(())
  }
}

fn main() {
    // Create a new application config
    let config = Config::new()
                .title("My Game".to_owned());

    // Run the application
    match atlas::event::run(config) {
      Ok(_) => exit(0),
      Err(e) => panic!("{:?}", e),
    }
}
```
More examples of how to use Atlas and some of it's great features can be found in the [examples directory in the Atlas repository](https://github.com/nurodev/atlas/tree/master/examples/)

## Contributions:
This project is 100% open source and all contributions are welcome.
If you think of something you'd like to add, or find something that needs to be tweaked or fixed,
feel free to submit a PR.
*/

#![crate_name = "atlas"]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/NuroDev/atlas/master/assets/Logo.png"
)]
#![warn(dead_code)]

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate quick_error;

// Core Modules
pub mod config;
pub mod error;
pub mod event;
pub mod graphics;

// Export Modules
pub extern crate nalgebra as math;
