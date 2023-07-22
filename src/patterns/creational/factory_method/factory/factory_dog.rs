use crate::patterns::creational::factory_method::product::{base::Animal, dog::Dog};
use super::base::Factory;

// Dog工厂
pub(crate) struct DogFactory {}

impl Factory for DogFactory {
    fn new() -> Box<dyn Animal> {
        Box::new(Dog::new())
    }
}