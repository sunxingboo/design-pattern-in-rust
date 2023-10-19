use super::concrete_visitable_object::ResidentialBuilding;
use super::concrete_visitable_object::BankBuilding;

pub trait Visitor {
	fn visit_residential_building(&self, _: &ResidentialBuilding);
	fn visit_bank_building(&self, _: &BankBuilding);
}