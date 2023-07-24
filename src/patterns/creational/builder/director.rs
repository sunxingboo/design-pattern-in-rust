use crate::patterns::creational::builder::builder::base::Builder;

// 引导器
pub struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    pub fn new(builder: Box<dyn Builder>) -> Self {
        Director {
            builder,
        }
    }

    pub fn get_builder(&self) -> Box<dyn Builder> {
        self.builder
    }

    pub fn change_builder(&mut self, builder: Box<dyn Builder>) -> &mut Self {
        self.builder = builder;

        self
    }

    pub fn make(&mut self) {
        self.builder.as_mut().set_appearance();
        self.builder.as_mut().set_attribute();
    }
}