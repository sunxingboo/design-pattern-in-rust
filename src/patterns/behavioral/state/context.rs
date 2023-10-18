use std::cell::RefCell;
use std::rc::Rc;
use super::state::State;
use super::concrete_state::Draft;

pub struct Document {
	author: String,
	state: Option<Rc<RefCell<dyn State>>>,
}

impl Document {
	/// 新建文档默认为草稿状态
	pub fn new(author: String) -> Self {
		let mut ctx = Document { author, state: None };

		let state = Draft::new(
			Some(
				Rc::new(
					RefCell::new(&ctx)
				)
			)
		);

		ctx.set_state(Some(
			Rc::new(
				RefCell::new(state)
			)
		));

		ctx
	}

	pub fn set_state(&mut self, state: Option<Rc<RefCell<dyn State>>>) {
		self.state = state;
	}

	pub fn author_is_admin(&self) -> bool {
		match self.author.as_str() {
			"admin" => true,
			_ => false
		}
	}

	pub fn publish(&self) {
		self.state.as_ref().unwrap().borrow().publish();
	}
}