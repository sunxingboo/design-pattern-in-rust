mod mediator;
mod colleague;

#[cfg(test)]
mod tests {
	use std::cell::RefCell;
	use std::rc::Rc;
	use crate::patterns::behavioral::mediator::colleague::{AirBus320, Boeing737};
	use super::mediator::ControlTower;

	#[test]
	fn base() {
		let mediator = Rc::new(
			RefCell::new(ControlTower::new())
		);

		let air_plane1 = AirBus320::new(mediator.clone());
		let air_plane2 = Boeing737::new(mediator.clone());

		mediator.borrow_mut().
			add_member(Box::new(air_plane1)).
			add_member(Box::new(air_plane2));

		mediator.borrow().air_planes[0].apply_for_landing();
		mediator.borrow().air_planes[1].apply_for_landing();
	}
}