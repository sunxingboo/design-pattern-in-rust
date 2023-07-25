mod factory;
mod product;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let cat = factory::new("cat");
        let dog = factory::new("dog");

        println!("cat: {}", cat.say());
        println!("dog: {}", dog.say());
    }
}