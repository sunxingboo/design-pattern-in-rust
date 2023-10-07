/// 药房。根据医生处方单配置药物。
pub struct Pharmacy;

impl Pharmacy {
	pub fn new() -> Self {
		Pharmacy
	}

	pub fn prescribe_medication(&self) {
		println!("已根据医生处方单配置药物。");
	}
}