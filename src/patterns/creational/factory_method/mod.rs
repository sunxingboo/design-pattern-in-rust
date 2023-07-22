pub(crate) mod factory;
mod product;

#[cfg(test)]
mod tests {
    use super::factory::base::Factory;
    use super::factory::factory_cat::CatFactory;
    use super::factory::factory_dog::DogFactory;

    #[test]
    fn base() {
        let cat = CatFactory::new();
        let dog = DogFactory::new();

        println!("cat: {}", cat.say());
        println!("dog: {}", dog.say());
    }
}