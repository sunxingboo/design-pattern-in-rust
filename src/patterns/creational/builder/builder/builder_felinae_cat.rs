use crate::patterns::creational::builder::product::product_felinae_cat::FelinaeCat;

use super::base::Builder;

// 金渐层猫咪生成器
pub struct FelinaeCatBuilder {
    pub product: Box<FelinaeCat>,
}

impl FelinaeCatBuilder {
    pub fn new() -> Self {
        FelinaeCatBuilder{
            product: Box::new(FelinaeCat::default()),
        }
    }

    pub fn get_result(&self) -> &FelinaeCat {
        self.product.as_ref()
    }
}

impl Builder for FelinaeCatBuilder {
    fn set_appearance(&mut self) {
        self.product.as_mut().set_color("golden".to_string());
    }

    fn set_attribute(&mut self) {
        let p = self.product.as_mut();

        p.set_name("mimi".to_string());
        p.set_age(99);
    }
}