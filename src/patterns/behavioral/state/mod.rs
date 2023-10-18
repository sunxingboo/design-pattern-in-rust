mod context;
mod concrete_state;
mod state;

#[cfg(test)]
mod tests {
	use crate::patterns::behavioral::state::context::Document;

	#[test]
	fn base() {
		let d = Document::new("admin".to_string());
		d.publish();
	}
}