mod flyweight;
mod flyweight_factory;
mod concrete_flyweight;

#[cfg(test)]
mod tests {
    use crate::patterns::structural::flyweight::flyweight_factory::FlyweightFactory;

    #[test]
    fn base() {
        let mut f = FlyweightFactory::new();

        let fw1 = f.get_flyweight("one");
        fw1.operate();

        let fw2 = f.get_flyweight("two");
        fw2.operate();

        let fw3 = f.get_flyweight("two");
        fw3.operate();
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let mut f = FlyweightFactory::new();

        let fw = f.get_flyweight("three");
        fw.operate();
    }
}