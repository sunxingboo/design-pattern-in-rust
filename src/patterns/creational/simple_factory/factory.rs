use super::cat::Cat;
use super::dog::Dog;

// 工厂产品接口
pub trait Animal {
    fn say(&self) -> String;
}

/// 工厂生产方法，根据flag产出不同对象
/// 
/// # Examples
/// 
/// ```
/// let a = factory::new("cat")
/// ```
pub fn new(flag: &str) -> Box<dyn Animal> {
    match flag {
        "cat" => Box::new(Cat::new()),
        "dog" => Box::new(Dog::new()),
        _ => panic!("flag is invalid"),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::types::type_of;
    use super::*;

    #[test]
    fn base() {
        let cat = new("cat");
        let dog = new("dog");
    }
}