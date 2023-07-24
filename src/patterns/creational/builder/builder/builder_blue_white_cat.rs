use crate::patterns::creational::builder::product::product_blue_white_cat::BlueWhiteCat;
use super::base::Builder;

// 蓝白猫咪生成器
pub struct BlueWhiteCatBuilder {
    pub product: Box<BlueWhiteCat>,
}

impl BlueWhiteCatBuilder {
    pub fn new() -> Self {
        BlueWhiteCatBuilder{
            product: Box::new(BlueWhiteCat::default()),
        }
    }

    pub fn get_result(&self) -> &BlueWhiteCat {
        self.product.as_ref()
    }
}

impl Builder for BlueWhiteCatBuilder {
    fn set_appearance(&mut self) {
        self.product.as_mut().set_color("blue white".to_string());
    }

    fn set_attribute(&mut self) {
        let p = self.product.as_mut();
        
        p.set_name("mimi".to_string());
        p.set_age(100);
    }
}