/// 收费处。
pub(crate) struct Cashier;

impl Cashier {
	pub(crate) fn new() -> Self {
		Cashier
	}

	pub(crate) fn register(&self) {
		println!("挂号成功。");
	}

	pub(crate) fn pay(&self) {
		println!("缴费成功。");
	}

	pub(crate) fn print_the_ticket(&self) {
		println!("票据已打印。");
	}
}