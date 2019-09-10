use derive_new::new;

/// TODO: Documentation
#[derive(Clone, Debug)]
pub struct Resolution {
	/// TODO: Documentation
	pub width: u32,
	/// TODO: Documentation
	pub height: u32,
}

impl Default for Resolution {
	fn default() -> Resolution {
		Resolution {
			width: 1024,
			height: 768,
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
}

impl Default for Config {
	fn default() -> Config {
		Config {
			icon: String::new(),
			resizable: false,
			resolution: Resolution::default(),
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

	/// Set whether the window is resizable
	pub fn resizable(mut self, resizable: bool) -> Self {
		self.resizable = resizable;
		self
	}

	/// Set the window resolution (width/height)
	pub fn resolution(mut self, width: u32, height: u32) -> Self {
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
}
