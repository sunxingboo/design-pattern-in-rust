mod director;
mod builder;
mod product;


#[cfg(test)]
mod tests {
    use super::builder::{builder_blue_white_cat::BlueWhiteCatBuilder, builder_felinae_cat::FelinaeCatBuilder};
    use super::director::Director;
    

    #[test]
    fn base() {
        let mut d = Director::new(Box::new(b));

        d.make();
        println!("cat1: {}", d..get_result().say())
        
    }
}