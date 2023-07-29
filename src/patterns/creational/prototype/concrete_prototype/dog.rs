use crate::patterns::creational::prototype::prototype::Prototype;

// Cat 猫咪
#[derive(Default)]
pub struct Dog {
    color: String,
    hair: String,
}

impl Dog {
    pub fn new(color: String, hair: String) -> Self {
        Dog { color, hair }
    }

    pub fn signature(&self) -> String {
        format!("I'm a dog, I covered in {} {}.", self.color, self.hair)
    }
}

impl Prototype for Dog {
    type AssociateType = Dog;

    fn clone(&self) -> Self {
        Dog {
            color: self.color.clone(),
            hair: self.hair.clone(),
        }
    }
}