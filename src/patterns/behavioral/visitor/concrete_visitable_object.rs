use super::visitable::Building;
use super::visitor::Visitor;

/// 居民楼
pub struct ResidentialBuilding {
	residents: Vec<String>,
}

impl Building for ResidentialBuilding {
	fn accept(&self, visitor: &dyn Visitor) {
		visitor.visit_residential_building(self);
	}
}

impl ResidentialBuilding {
	pub fn new() -> Self {
		ResidentialBuilding {
			residents: vec![
				"Bob".to_string(),
			]
		}
	}

	pub fn get_residents(&self) -> Vec<String> {
		self.residents.clone()
	}
}

/// 银行
pub struct BankBuilding {
	staff: Vec<String>,
}

impl Building for BankBuilding {
	fn accept(&self, visitor: &dyn Visitor) {
		visitor.visit_bank_building(self);
	}
}

impl BankBuilding {
	pub fn new() -> Self {
		BankBuilding {
			staff: vec![
				"Mary".to_string(),
			]
		}
	}

	pub fn get_staff(&self) -> Vec<String> {
		self.staff.clone()
	}
}