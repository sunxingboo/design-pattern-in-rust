use super::state::State;
use super::context::Document;

/// 草稿状态
pub struct Draft;

impl State for Draft {
	fn handle(&self, ctx: &Document) -> Box<dyn State> {
		if ctx.author_is_admin() {
			println!("author is admin, state changed: draft -> published.");
			return Box::new(Published::new());
		}

		println!("state changed: draft -> moderation.");
		Box::new(Moderation::new())
	}
}

impl Draft {
	pub fn new() -> Self {
		Draft
	}
}

/// 审核状态
pub struct Moderation;

impl State for Moderation {
	fn handle(&self, ctx: &Document) -> Box<dyn State> {
		if ctx.author_is_admin() {
			println!("author is admin, state changed: draft -> published.");

			return Box::new(Published::new());
		}

		panic!("permission denied.");
	}
}

impl Moderation {
	pub fn new() -> Self {
		Moderation
	}
}

/// 已发布状态
pub struct Published;

impl State for Published {
	fn handle(&self, _: &Document) -> Box<dyn State> {
		println!("document is already published. do nothing.");
		Box::new(Published::new())
	}
}

impl Published {
	pub fn new() -> Self {
		Published
	}
}