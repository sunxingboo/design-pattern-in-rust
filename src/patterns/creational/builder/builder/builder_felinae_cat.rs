use crate::patterns::creational::builder::product::product_felinae_cat::FelinaeCat;

use super::base::Builder;

// 金渐层猫咪生成器
pub struct FelinaeCatBuilder {
    product: FelinaeCat,
}

impl FelinaeCatBuilder {
    pub fn new() -> Self {
        FelinaeCatBuilder{
            product: FelinaeCat::default(),
        }
    }

    pub fn get_result(&self) -> &FelinaeCat {
        &self.product
    }
}

impl Builder for FelinaeCatBuilder {
    type AssociateType = FelinaeCatBuilder;

    fn set_appearance(&mut self) {
        self.product.set_color("golden".to_string());
    }

    fn set_attribute(&mut self) {
        let p = &mut self.product;

        p.set_name("mimi".to_string());
        p.set_age(99);
    }
}