quick_error! {
	#[derive(Debug)]
	pub enum AtlasError {
		Event(message: String) {
		  description("Text")
		  display("Text: {}", message)
		}
	}
}
