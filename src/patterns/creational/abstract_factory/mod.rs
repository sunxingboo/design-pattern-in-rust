mod factory;
mod product;


#[cfg(test)]
mod tests {
    use crate::patterns::creational::abstract_factory::factory::base::Factory;
    use super::factory::{factory_blue_white_and_calico_cat::BlueWhiteAndCalicoCatFactory, factory_felinae_and_giner_cat::FelinaeAndGinerCat};

    #[test]
    fn base() {
        println!("BlueWhiteAndCalicoCatFactory:");
        let cat1 = BlueWhiteAndCalicoCatFactory::new_british_short_hair_cat("cat1".to_string());
        let cat2 = BlueWhiteAndCalicoCatFactory::new_pastoral_cat("cat2".to_string());
        println!("{}", cat1.say());
        println!("{}", cat2.say());

        
        println!("FelinaeAndGinerCat:");
        let cat1 = FelinaeAndGinerCat::new_british_short_hair_cat("cat1".to_string());
        let cat2 = FelinaeAndGinerCat::new_pastoral_cat("cat2".to_string());
        println!("{}", cat1.say());
        println!("{}", cat2.say());
    }
}