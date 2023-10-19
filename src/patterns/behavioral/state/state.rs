use super::context::Document;

pub trait State {
	fn handle(&self, _: &Document) -> Box<dyn State>;
}