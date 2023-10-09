mod component;
mod decorator;
mod concrete_component;

#[cfg(test)]
mod tests {
    use super::concrete_component::BaseNotifier;
    use super::component::Notifier;
    use super::decorator::{SMSDecorator, QQDecorator};

    #[test]
    fn base() {
        let d0 = BaseNotifier::new();
        let d1 = SMSDecorator::new(d0);
        let d2 = QQDecorator::new(d1);

        d2.notify("Alert!");
    }
}