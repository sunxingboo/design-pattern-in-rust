use crate::patterns::creational::builder::product::product_blue_white_cat::BlueWhiteCat;
use super::base::Builder;

// 蓝白猫咪生成器
pub struct BlueWhiteCatBuilder {
    product: BlueWhiteCat,
}

impl BlueWhiteCatBuilder {
    pub fn new() -> Self {
        BlueWhiteCatBuilder{
            product: BlueWhiteCat::default(),
        }
    }

    pub fn get_result(&self) -> &BlueWhiteCat {
        &self.product
    }
}

impl Builder for BlueWhiteCatBuilder {
    type AssociateType = BlueWhiteCatBuilder;

    fn set_appearance(&mut self) {
        self.product.set_color("blue white".to_string());
    }

    fn set_attribute(&mut self) {
        let p = &mut self.product;
        
        p.set_name("mimi".to_string());
        p.set_age(100);
    }
}