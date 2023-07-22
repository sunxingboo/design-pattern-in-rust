use super::product::base::Animal;
use super::product::cat::Cat;
use super::product::dog::Dog;

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
    use super::*;

    #[test]
    fn base() {
        let cat = new("cat");
        let dog = new("dog");

        println!("cat: {}", cat.say());
        println!("dog: {}", dog.say());
    }
}