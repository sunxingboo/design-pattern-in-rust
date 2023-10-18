use std::cell::RefCell;
use std::rc::Rc;
use super::state::State;
use super::context::Document;

/// 草稿状态
pub struct Draft<'a> {
	ctx: Option<Rc<RefCell<&'a Document>>>,
}

impl State for Draft<'_> {
	fn publish(&self) {
		let mut ctx = self.ctx.as_ref().unwrap().borrow_mut();

		if ctx.author_is_admin() {
			ctx.set_state(
				Some(Rc::new(
					RefCell::new(
						Published::new(self.ctx.clone())
					)
				))
			);

			println!("author is admin, state changed: draft -> published.");
			return;
		}

		ctx.set_state(
			Some(Rc::new(
				RefCell::new(
					Moderation::new(self.ctx.clone())
				)
			))
		);

		println!("state changed: draft -> moderation.");
	}
}

impl Draft<'_> {
	pub fn new(ctx: Option<Rc<RefCell<&Document>>>) -> Self {
		Draft {
			ctx
		}
	}
}

/// 审核状态
pub struct Moderation<'a> {
	ctx: Option<Rc<RefCell<&'a Document>>>
}

impl State for Moderation<'_> {
	fn publish(&self) {
		let mut ctx = self.ctx.unwrap().borrow();

		if ctx.author_is_admin() {
			ctx.set_state(
				Some(Rc::new(
					RefCell::new(
						Published::new(self.ctx.clone())
					)
				))
			);

			println!("author is admin, state changed: draft -> published.");
			return;
		}

		panic!("permission denied.");
	}
}

impl Moderation<'_> {
	pub fn new(ctx: Option<Rc<RefCell<&Document>>>) -> Self {
		Moderation {
			ctx
		}
	}
}

/// 已发布状态
pub struct Published<'a> {
	ctx: Option<Rc<RefCell<&'a Document>>>
}

impl State for Published<'_> {
	fn publish(&self) {
		println!("already published.");
	}
}

impl Published<'_> {
	pub fn new(ctx: Option<Rc<RefCell<&Document>>>) -> Self {
		Published {
			ctx
		}
	}
}