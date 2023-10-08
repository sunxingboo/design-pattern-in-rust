mod abstraction;
mod refined_abstraction;
mod implementor;
mod concrete_implementor;

#[cfg(test)]
mod tests {
    use super::abstraction::Shape;
    use super::concrete_implementor::{GreenCircle, RedCircle};
    use super::refined_abstraction::Circle;

    #[test]
    fn base() {
        let c1 = Circle::new(1, 1, 1, Box::new(RedCircle::new()));
        let c2 = Circle::new(7, 7, 7, Box::new(GreenCircle::new()));

        c1.draw();
        c2.draw();
    }
}