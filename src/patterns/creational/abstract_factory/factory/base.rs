use crate::patterns::creational::abstract_factory::product::base::{BritishShorthairCat, PastoralCat};

// 抽象工厂
pub trait Factory {
    fn new_british_short_hair_cat(name: String) -> Box<dyn BritishShorthairCat>;
    fn new_pastoral_cat(name: String) -> Box<dyn PastoralCat>;
}