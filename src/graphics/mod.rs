use crate::Result;
use imgui::Ui;

/// The interface between the engine and your game.
/// Implement it, and pass your struct to `Window::main_loop`.
pub trait Game {
	/// Called every frame to draw/render content
	fn draw(&mut self) -> Result;
	/// Called on window close button click
	fn exit(&mut self) -> Result;
	/// Called at launch to initialize & load assets
	fn load(&mut self) -> Result;
	/// Draw custom UI elements using imgui
	fn ui(&mut self, ui: &mut Ui) -> Result;
	/// Called on every update tick event
	fn update(&mut self) -> Result;
}

mod renderer;
mod types;

pub use renderer::AtlasRenderer as Renderer;
pub use types::*;
