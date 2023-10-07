/// 收费处。
pub struct Cashier;

impl Cashier {
	pub fn new() -> Self {
		Cashier
	}

	pub fn register(&self) {
		println!("挂号成功。");
	}

	pub fn pay(&self) {
		println!("缴费成功。");
	}

	pub fn print_the_ticket(&self) {
		println!("票据已打印。");
	}
}