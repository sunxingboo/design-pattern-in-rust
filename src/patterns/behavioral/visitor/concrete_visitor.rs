use crate::patterns::behavioral::visitor::concrete_visitable_object::{BankBuilding, ResidentialBuilding};
use super::visitor::Visitor;

/// 销售人员
pub struct Saler;

impl Visitor for Saler {
	fn visit_residential_building(&self, building: &ResidentialBuilding) {
		for i in building.get_residents() {
			println!("selling life insurance to {}", i)
		}
	}

	fn visit_bank_building(&self, building: &BankBuilding) {
		for i in building.get_staff() {
			println!("selling theft insurance to {}", i)
		}
	}
}

impl Saler {
	pub fn new() -> Self {
		Saler
	}
}