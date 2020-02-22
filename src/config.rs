use derive_new::new;

/// TODO: Documentation
#[derive(Clone, Debug)]
pub enum WindowMode {
	/// Free floating window
	Borderless,
	/// Complete fullscreen window
	Fullscreen,
	/// Fake fullscreen (Plays better with alt tabbing/Multiple monitors) window
	Windowed,
}

impl Default for WindowMode {
	fn default() -> WindowMode { WindowMode::Windowed }
}

/// TODO: Documentation
#[derive(Clone, Debug)]
pub struct Resolution {
	/// TODO: Documentation
	pub width: f64,
	/// TODO: Documentation
	pub height: f64,
}

impl Default for Resolution {
	fn default() -> Resolution {
		Resolution {
			width: 1024f64,
			height: 768f64,
		}
	}
}

/// TODO: Documentation
#[derive(Clone, Debug, new)]
pub struct Config {
	/// Path for the application icon
	#[new(default)]
	pub icon: String,

	/// If the window resizable
	#[new(default)]
	pub resizable: bool,

	/// Window resolution (Width x Height)
	#[new(default)]
	pub resolution: Resolution,

	/// Window title
	#[new(default)]
	pub title: String,

	/// Whether to enable v-sync or not
	#[new(default)]
	pub vsync: bool,

	/// Window mode
	#[new(default)]
	pub window_mode: WindowMode,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			icon: String::new(),
			resizable: false,
			resolution: Resolution::default(),
			title: "Atlas".to_owned(),
			vsync: true,
			window_mode: WindowMode::Windowed,
		}
	}
}

impl Config {
	/// Set the application icon
	pub fn icon(mut self, icon: String) -> Self {
		self.icon = icon;
		self
	}

	/// Set whether the window is resizable
	pub fn resizable(mut self, resizable: bool) -> Self {
		self.resizable = resizable;
		self
	}

	/// Set the window resolution (width/height)
	pub fn resolution(mut self, width: f64, height: f64) -> Self {
		self.resolution = Resolution {
			width,
			height,
		};
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
