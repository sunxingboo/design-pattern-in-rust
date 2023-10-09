use super::component::Component;

pub struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operate(&self) -> String {
        format!("ConcreteComponent")
    }
}

impl ConcreteComponent {
    pub fn new() -> Self {
        ConcreteComponent
    }
}