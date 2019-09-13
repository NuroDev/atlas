use quick_error::quick_error;

quick_error! {
	#[derive(Debug)]
	pub enum AtlasError {
		/// TODO: Documentation
		Audio(message: String) {
		  description("Audio Error")
		  display("Audio Error: {}", message)
		}
		/// TODO: Documentation
		Config(message: String) {
		  description("Config Error")
		  display("Config Error: {}", message)
		}
		/// TODO: Documentation
		Event(message: String) {
		  description("Event Error")
		  display("Event Error: {}", message)
		}
		/// TODO: Documentation
		Font(message: String) {
		  description("Font Error")
		  display("Font Error: {}", message)
		}
		/// TODO: Documentation
		Graphics(message: String) {
		  description("Graphics Error")
		  display("Graphics Error: {}", message)
		}
		/// TODO: Documentation
		Input(message: String) {
		  description("Input Error")
		  display("Input Error: {}", message)
		}
		/// TODO: Documentation
		Resource(message: String) {
		  description("Resource Error")
		  display("Resource Error: {}", message)
		}
		/// TODO: Documentation
		Window(message: String) {
		  description("Window Error")
		  display("Window Error: {}", message)
		}
	}
}
