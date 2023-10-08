use std::collections::HashMap;
use std::rc::Rc;
use crate::patterns::structural::flyweight::concrete_flyweight::{ConcreteFlyweightOne, ConcreteFlyweightTwo};
use super::flyweight::Flyweight;

pub struct FlyweightFactory {
    pool: HashMap<String, Rc<dyn Flyweight>>
}

impl FlyweightFactory {
    pub fn new() -> Self {
        FlyweightFactory {
            pool: HashMap::new(),
        }
    }

    // TODO：此处的实现方式使FlyweightFactory与具体的Flyweight耦合，是否与类图不符？
    pub fn get_flyweight(&mut self, key: &str) -> Rc<dyn Flyweight> {
        if !self.pool.contains_key(key) {
            let flyweight =  match key {
                "one" => Rc::new(ConcreteFlyweightOne::new()) as Rc<dyn Flyweight>,
                "two" => Rc::new(ConcreteFlyweightTwo::new()) as Rc<dyn Flyweight>,
                _=> panic!("invalid key."),
            };

            self.pool.insert(key.to_string(), flyweight);
        } else {
            println!("key {} is exist.", key);
        }

        self.pool[key].clone()
    }
}