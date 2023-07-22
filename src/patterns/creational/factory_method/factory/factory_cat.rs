use crate::patterns::creational::factory_method::product::{base::Animal, cat::Cat};
use super::base::Factory;

// Cat工厂
pub(crate) struct CatFactory {}

impl Factory for CatFactory {
    fn new() -> Box<dyn Animal> {
        Box::new(Cat::new())
    }
}