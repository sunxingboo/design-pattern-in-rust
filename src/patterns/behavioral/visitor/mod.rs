mod visitor;
mod visitable;
mod concrete_visitor;
mod concrete_visitable_object;

#[cfg(test)]
mod tests {
	use crate::patterns::behavioral::visitor::concrete_visitable_object::{BankBuilding, ResidentialBuilding};
	use crate::patterns::behavioral::visitor::concrete_visitor::Saler;
	use crate::patterns::behavioral::visitor::visitable::Building;

	#[test]
	fn base() {
		let v = Saler::new();
		let rb = ResidentialBuilding::new();
		let bb = BankBuilding::new();

		rb.accept(&v);
		bb.accept(&v);
	}
}