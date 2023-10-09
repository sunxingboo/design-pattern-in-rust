use super::component::Notifier;

/// 基础通知组件。定义了最基础的通知组件的行为。
pub struct BaseNotifier;

impl Notifier for BaseNotifier {
	fn notify(&self, msg: &str) {
		println!("send email: {}", msg);
	}
}

impl BaseNotifier {
	pub fn new() -> Self {
		BaseNotifier
	}
}
