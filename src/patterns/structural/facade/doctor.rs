/// 医生。负责诊断病情、制定处方单。
pub struct Doctor;

impl Doctor {
	pub fn new() -> Self {
		Doctor
	}

	pub fn diagnosis(&self) {
		println!("诊断完成，针对病情制定药品处方单。");
	}
}