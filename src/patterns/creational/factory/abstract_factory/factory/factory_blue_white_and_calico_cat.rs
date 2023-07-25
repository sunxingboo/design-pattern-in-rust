use super::base::Factory;
use crate::patterns::creational::factory::abstract_factory::product::base::{BritishShorthairCat, PastoralCat};
use crate::patterns::creational::factory::abstract_factory::product::product_british_shor_hair_cat::BlueWhiteCat;
use crate::patterns::creational::factory::abstract_factory::product::product_pastoral_cat::CalicoCat;

// 英短猫咪工厂
#[derive(Default)]
pub struct BlueWhiteAndCalicoCatFactory {}

// 实现抽象工厂接口，生产蓝白英短猫咪和三花田园猫咪
impl Factory for BlueWhiteAndCalicoCatFactory {
    // 生产英短猫
    fn new_british_short_hair_cat(name: String) -> Box<dyn BritishShorthairCat> {
        Box::new(BlueWhiteCat::new(name))
    }

    // 生产田园猫
    fn new_pastoral_cat(name: String) -> Box<dyn PastoralCat> {
        Box::new(CalicoCat::new(name))
    }
}