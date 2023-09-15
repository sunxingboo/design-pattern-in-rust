use std::cell::RefCell;
use std::rc::Rc;
use super::mediator::Mediator;

/// 系统中需要中介者来协调的成员；
pub(crate) trait Colleague {
	fn id(&self) -> i32;
	fn apply_for_landing(&self);
	fn receive(&self, msg: &str);
}

/// 空客320
pub(crate) struct AirBus320 {
	mediator: Rc<RefCell<dyn Mediator>>,
}

impl AirBus320 {
	pub fn new(mediator: Rc<RefCell<dyn Mediator>>) -> Self {
		AirBus320 {
			mediator,
		}
	}
}

impl Colleague for AirBus320 {
	fn id(&self) -> i32 {
		320
	}

	fn apply_for_landing(&self) {
		self.mediator.borrow().notify(self);
	}

	fn receive(&self, msg: &str) {
		println!("AirBus320 received: {}", msg);
	}
}

/// 波音737
pub(crate) struct Boeing737 {
	mediator: Rc<RefCell<dyn Mediator>>,
}

impl Boeing737 {
	pub fn new(mediator: Rc<RefCell<dyn Mediator>>) -> Self {
		Boeing737 {
			mediator,
		}
	}
}

impl Colleague for Boeing737 {
	fn id(&self) -> i32 {
		737
	}

	fn apply_for_landing(&self) {
		self.mediator.borrow().notify(self);
	}

	fn receive(&self, msg: &str) {
		println!("Boeing737 received: {}", msg);
	}
}