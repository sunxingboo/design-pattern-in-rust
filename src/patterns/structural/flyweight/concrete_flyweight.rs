use super::flyweight::Flyweight;

/// 具体享元对象
pub struct ConcreteFlyweightOne;

impl Flyweight for ConcreteFlyweightOne {
    fn operate(&self) {
        println!("category: concrete flyweight one.");
    }
}

impl ConcreteFlyweightOne {
    pub fn new() -> Self {
        ConcreteFlyweightOne
    }
}

/// 具体享元对象
pub struct ConcreteFlyweightTwo;

impl Flyweight for ConcreteFlyweightTwo {
    fn operate(&self) {
        println!("category: concrete flyweight two.");
    }
}

impl ConcreteFlyweightTwo {
    pub fn new() -> Self {
        ConcreteFlyweightTwo
    }
}