use super::component::Component;

pub struct  DecoratorA<T: Component> {
    component: T,
}

impl<T: Component> Component for DecoratorA<T> {
    fn operate(&self) -> String {
        format!("DecoratorA[ {} ]", self.component.operate())
    }
}

impl<T: Component> DecoratorA<T> {
    pub fn new(component: T) -> DecoratorA<T> {
        DecoratorA{
            component
        }
    }
}

pub struct  DecoratorB<T: Component> {
    component: T,
}

impl<T: Component> Component for DecoratorB<T> {
    fn operate(&self) -> String {
        format!("DecoratorB[ {} ]", self.component.operate())
    }
}

impl<T: Component> DecoratorB<T> {
    pub fn new(component: T) -> DecoratorB<T> {
        DecoratorB{
            component
        }
    }
}