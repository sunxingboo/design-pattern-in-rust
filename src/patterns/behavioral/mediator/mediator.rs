use super::colleague::Member;

/// 中介者。协调多个对象的交互，使对象间不再显示的调用，以此解耦。
pub(crate) trait Mediator {
	fn notify(&self, _: &dyn Member);
}

/// 塔台。
pub(crate) struct ControlTower {
	pub(crate) air_planes: Vec<Box<dyn Member>>,
}

impl ControlTower {
	pub fn new() -> Self {
		ControlTower {
			air_planes: vec![],
		}
	}

	pub fn add_member(&mut self, air_plane: Box<dyn Member>) -> &mut Self {
		self.air_planes.push(air_plane);
		self
	}
}

impl Mediator for ControlTower {
	fn notify(&self, sender: &dyn Member) {
		let mut gate = 0;
		for i in 0..self.air_planes.len() {
			if self.air_planes[i].id() == sender.id() {
				gate = i;
			}
		}

		for i in &self.air_planes {
			if i.id() != sender.id() {
				let msg = format!("{} landed at position {}", sender.id(), gate);
				i.receive(msg.as_str());
			}
		}
	}
}
