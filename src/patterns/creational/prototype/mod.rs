mod prototype;
mod concrete_prototype;


#[cfg(test)]
mod tests {
    use crate::patterns::creational::prototype::concrete_prototype::{cat::Cat, dog::Dog};
    use crate::patterns::creational::prototype::prototype::Prototype;

    #[test]
    fn base() {
        let c = Cat::new("mimi".to_string(), 100);
        assert_eq!(c.signature(), c.clone().signature());

        let d = Dog::new("golden".to_string(), "long hair".to_string());
        assert_eq!(d.signature(), d.clone().signature());

    }
}