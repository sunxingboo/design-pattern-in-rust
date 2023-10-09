mod component;
mod concrete_component;
mod decorator;

#[cfg(test)]
mod tests {
    use super::component::Component;
    use super::concrete_component::ConcreteComponent;
    use super::decorator::{DecoratorA, DecoratorB};

    #[test]
    fn base() {
        let c = ConcreteComponent::new();
        let d1 = DecoratorA::new(c);
        let d2 = DecoratorB::new(d1);

        println!("{}", d2.operate());
    }
}