mod context;
mod concrete_state;
mod state;

#[cfg(test)]
mod tests {
	use crate::patterns::behavioral::state::context::Document;

	#[test]
	fn base() {
		let mut doc = Document::new("admin".to_string());
		let next_state = doc.publish();
		doc.set_state(next_state);
		doc.publish();
	}

	#[test]
	#[should_panic]
	fn test_exception() {
		let mut doc = Document::new("aaa".to_string());
		let next_state = doc.publish();
		doc.set_state(next_state);
		doc.publish();
	}
}