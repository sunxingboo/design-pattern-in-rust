use super::visitor::Visitor;

pub trait Building {
	fn accept(&self, _: &dyn Visitor);
}