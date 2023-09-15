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
	/// 这里的实现是当一架飞机发送降落请求时，由塔台分配降落的机位，并通知其他飞机。
	/// 但是这个例子并没有太确切的说明中介者模式。
	///
	/// 总之，在中介者模式中，请求由「某一个具体的成员」发起，经过中介者转发（可能由一些必要的业务逻辑处理），
	/// 最终由某一个或某几个其他成员对象接收。
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
