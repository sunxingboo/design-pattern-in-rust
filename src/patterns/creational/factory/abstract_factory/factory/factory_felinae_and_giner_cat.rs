use super::base::Factory;
use crate::patterns::creational::factory::abstract_factory::product::base::{BritishShorthairCat, PastoralCat};
use crate::patterns::creational::factory::abstract_factory::product::product_british_shor_hair_cat::FelinaeCat;
use crate::patterns::creational::factory::abstract_factory::product::product_pastoral_cat::GingerCat;

#[derive(Default)]
pub struct FelinaeAndGinerCat {}

// 实现抽象工厂接口，生产金渐层英短猫咪和橘色田园猫咪
impl Factory for FelinaeAndGinerCat {
    // 生产金渐层猫
    fn new_british_short_hair_cat(name: String) -> Box<dyn BritishShorthairCat> {
        Box::new(FelinaeCat::new(name))
    }

    // 生产橘猫
    fn new_pastoral_cat(name: String) -> Box<dyn PastoralCat> {
        Box::new(GingerCat::new(name))
    }
}