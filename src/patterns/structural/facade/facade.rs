use super::cashier::Cashier;
use super::doctor::Doctor;
use super::pharmacy::Pharmacy;

/// 外观接口。
/// 这个一个复杂子系统的对外接口，以此隐藏系统内部的复杂性，使其简单易用。
/// 另外也减轻了其他模块对此系统的依赖，在进行重构或替换时，只需实现此接口即可。
pub(crate) trait Facade {
	fn treat(&self);
}

/// 医院。
/// 普通情况下，一个患者到医院需要：挂号->交费->报道->分诊->排队->治疗->打印票据。
/// 这就要求患者需要知道医院的整个流程，自己严格安装流程执行，但这样患者在使用「医院」这个子系统时就有比较
/// 大的心智负担，而通过医院的外接口，患者不需要知道系统的内部流程，只需要直接等待治疗，其他的事项均交给系统
/// 自动完成。
pub(crate) struct Hospital {
	cashier: Cashier,
	doctor: Doctor,
	pharmacy: Pharmacy,
}

impl Hospital {
	pub(crate) fn new() -> Self {
		Hospital {
			cashier: Cashier::new(),
			doctor: Doctor::new(),
			pharmacy: Pharmacy::new(),
		}
	}
}

impl Facade for Hospital {
	/// 一个简易化的流程，正常相互之间可能存在更多依赖，比如药房需要拿到医生开出的处方单，针对处方单的
	/// 每一条数据进行特殊处理。
	fn treat(&self) {
		self.cashier.register();
		self.cashier.pay();
		self.doctor.diagnosis();
		self.pharmacy.prescribe_medication();
		self.cashier.print_the_ticket();
	}
}