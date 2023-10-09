use super::component::Notifier;

/// SMS通知装饰器
pub struct SMSDecorator<T: Notifier> {
    notifier: T,
}

impl<T: Notifier> Notifier for SMSDecorator<T> {
    fn notify(&self, msg: &str) {
        self.notifier.notify(msg);
        println!("send SMS: {}", msg);
    }
}

impl<T: Notifier> SMSDecorator<T> {
    pub fn new(notifier: T) -> SMSDecorator<T> {
        SMSDecorator {
            notifier
        }
    }
}

/// QQ通知装饰器
pub struct QQDecorator<T: Notifier> {
    notifier: T,
}

impl<T: Notifier> Notifier for QQDecorator<T> {
    fn notify(&self, msg: &str) {
        self.notifier.notify(msg);
        println!("send QQ: {}", msg);
    }
}

impl<T: Notifier> QQDecorator<T> {
    pub fn new(notifier: T) -> QQDecorator<T> {
        QQDecorator {
            notifier
        }
    }
}