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
pub fn new(flag: &str, name: String) -> Box<dyn Animal> {
    match flag {
        "cat" => Box::new(Cat::new(name)),
        "dog" => Box::new(Dog::new(name)),
        _ => panic!("flag is invalid"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let cat = new("cat", "mimi".to_string());
        let dog = new("dog", "wangwang".to_string());

        println!("cat: {}", cat.say());
        println!("dog: {}", dog.say());
    }
}