/// 药房。根据医生处方单配置药物。
pub(crate) struct Pharmacy;

impl Pharmacy {
	pub(crate) fn new() -> Self {
		Pharmacy
	}

	pub(crate) fn prescribe_medication(&self) {
		println!("已根据医生处方单配置药物。");
	}
}