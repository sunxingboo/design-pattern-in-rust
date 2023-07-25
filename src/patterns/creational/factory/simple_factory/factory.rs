use super::product::base::Animal;
use super::product::product_cat::Cat;
use super::product::product_dog::Dog;

/// 工厂生产方法，根据flag产出不同对象
/// 
/// # Examples
/// 
/// ```
/// let a = factory::new("cat")
/// ```
pub fn new(flag: &str) -> Box<dyn Animal> {
    match flag {
        "cat" => Box::new(Cat::new("mimi".to_string())),
        "dog" => Box::new(Dog::new("wangwang".to_string())),
        _ => panic!("flag is invalid"),
    }
}