mod director;
mod builder;
mod product;


#[cfg(test)]
mod tests {
    use super::builder::{builder_blue_white_cat::BlueWhiteCatBuilder, builder_felinae_cat::FelinaeCatBuilder};
    use super::director::Director;
    

    #[test]
    fn base() {
        let bwcb = BlueWhiteCatBuilder::new();
        let mut d = Director::new(bwcb);

        d.build();
        println!("cat1: {}", d.get_builder().get_result().say());

        let fcb = FelinaeCatBuilder::new();
        let mut d = Director::new(fcb);
        
        d.build();
        println!("cat2: {}", d.get_builder().get_result().say());
    }
}