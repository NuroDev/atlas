/*!
[![](https://github.com/NuroDev/atlas/blob/master/assets/Atlas-Banner.png?raw=true)](https://github.com/nurodev/atlas)

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
atlas = { git ="https://github.com/nurodev/atlas" }
```

## Example:
```rust
extern crate atlas;

use atlas::Atlas;
use atlas::window::Window;

struct MyGame {
  // Game assets & state
}

impl Atlas for MyGame {
  fn load(&mut self) {
    // Load the application & any assets
  }

  fn draw(&mut self) {
    // Draw a new frame
  }
  
  fn update (&mut self) {
    // Update every tick
  }
}

fn main() -> Result<()> {
    // Create a new application window
    let window = Window::new()
                .title("My Game".to_owned())
                .resolution(1024, 768)
                .resizable(false)
                .fullscreenable(false);

    // Run the application
    match MyGame::run(window) {
      Ok(_) => exit(0),
      Err(e) => {
        error!("Error: {:?}", e);
        exit(1);
      },
    }
}
```
More examples of how to use Atlas and some of it's great features can be found in the [examples directory in the Atlas repository](https://github.com/nurodev/atlas/tree/master/examples/)

## Contributions:
This project is 100% open source and all contributions are welcome.
If you think of something you'd like to add, or find something that needs to be tweaked or fixed,
feel free to submit a PR.
*/

#![doc(html_logo_url = "https://raw.githubusercontent.com/NuroDev/atlas/master/assets/Atlas-Logo-Transparent.png")]
#![crate_name = "atlas"]
#![warn(dead_code)]

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate quick_error;

#[cfg(windows)]
extern crate gfx_backend_dx12 as backend;
#[cfg(target_os = "macos")]
extern crate gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan as backend;
extern crate gfx_hal as hal;

pub mod error;
pub mod event;

// Re-exports
pub use log::*;
pub use env_logger::*;
pub extern crate nalgebra as math;
