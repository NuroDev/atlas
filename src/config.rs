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
	/// Window height
	pub height: u32,
	/// Path for the application icon
	pub icon: String,
	/// Window mode
	pub mode: WindowMode,
	/// If the window resizable
	pub resizable: bool,
	/// Window title
	pub title: String,
	/// Whether to enable v-sync or not
	pub vsync: bool,
	/// Window width
	pub width: u32,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			height: 768,
			icon: String::new(),
			mode: WindowMode::Windowed,
			resizable: false,
			title: "Atlas".to_owned(),
			vsync: true,
			width: 1024,
		}
	}
}

impl Config {
	/// Set the application icon
	pub fn icon(mut self, icon: String) -> Self {
		self.icon = icon.to_owned();
		self
	}

	/// Set the window mode
	pub fn mode(mut self, mode: WindowMode) -> Self {
		self.mode = mode;
		self
	}

	///  Create new `Config` instance using `Config::Default`
	pub fn new() -> Self { Self::default() }

	/// Set whether the window is resizable
	pub fn resizable(mut self, resizable: bool) -> Self {
		self.resizable = resizable;
		self
	}

	/// Set the window resolution (width/height)
	pub fn resolution(mut self, width: u32, height: u32) -> Self {
		self.width = width;
		self.height = height;
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
}
