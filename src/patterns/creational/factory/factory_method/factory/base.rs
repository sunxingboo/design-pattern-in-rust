use crate::patterns::creational::factory::factory_method::product::base::Animal;

/// 抽象工厂
/// 定义产品的创建接口，不同的具体工厂类实现此接口以定制产品
pub trait Factory {
    fn new(name: String) -> Box<dyn Animal>;
}