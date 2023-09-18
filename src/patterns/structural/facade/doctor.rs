/// 医生。负责诊断病情、制定处方单。
pub(crate) struct Doctor;

impl Doctor {
	pub(crate) fn new() -> Self {
		Doctor
	}

	pub(crate) fn diagnosis(&self) {
		println!("诊断完成，针对病情制定药品处方单。");
	}
}