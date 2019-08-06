use hal::window::Extent2D;

/// TODO: Documentation
#[derive(Debug, Copy, Clone)]
pub enum WindowMode {
	/// Free floating window
	Borderless,
	/// Complete fullscreen window
	Fullscreen,
	/// Fake fullscreen (Plays better with alt tabbing/Multiple monitors) window
	Windowed,
}

/// TODO: Documentation
#[derive(Debug, Clone)]
pub struct Config {
	/// Path for the application icon
	pub icon: String,
	/// Window mode
	pub window_mode: WindowMode,
	/// If the window resizable
	pub resizable: bool,
	/// Window resolution (Width x Height)
	pub resolution: Extent2D,
	/// Window title
	pub title: String,
	/// Whether to enable v-sync or not
	pub vsync: bool,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			icon: String::new(),
			window_mode: WindowMode::Windowed,
			resizable: false,
			resolution: Extent2D {
				width: 1024,
				height: 768,
			},
			title: "Atlas".to_owned(),
			vsync: true,
		}
	}
}

impl Config {
	/// Set the application icon
	pub fn icon(mut self, icon: String) -> Self {
		self.icon = icon.to_owned();
		self
	}

	///  Create new `Config` instance using default values
	pub fn new() -> Self { Self::default() }

	/// Set whether the window is resizable
	pub fn resizable(mut self, resizable: bool) -> Self {
		self.resizable = resizable;
		self
	}

	/// Set the window resolution (width/height)
	pub fn resolution(mut self, resolution: Extent2D) -> Self {
		self.resolution = resolution;
		self
	}

	/// Set the window title
	pub fn title(mut self, title: String) -> Self {
		self.title = title;
		self
	}

	/// Set whether to use V-sync or not
	pub fn vsync(mut self, vsync: bool) -> Self {
		self.vsync = vsync;
		self
	}

	/// Set the window mode
	pub fn window_mode(mut self, window_mode: WindowMode) -> Self {
		self.window_mode = window_mode;
		self
	}
}
