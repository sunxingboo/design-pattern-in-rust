use crate::patterns::creational::prototype::prototype::Prototype;

// Cat 猫咪
#[derive(Default)]
pub struct Cat {
    name: String,
    age: i32,
}

impl Cat {
    pub fn new(name: String, age: i32) -> Self {
        Cat { name, age }
    }

    pub fn signature(&self) -> String {
        format!("I'm a cat, my name is {}, I'm {} years old.", self.name, self.age)
    }
}

impl Prototype for Cat {
    type AssociateType = Cat;

    fn clone(&self) -> Self {
        Cat {
            name: self.name.clone(),
            age: self.age.clone(),
        }
    }
}