mod context;
mod strategy;
mod concrete_strategy;

#[cfg(test)]
mod tests {
    use super::concrete_strategy::{RidingStrategy, SubwayStrategy};
    use super::context::Navigator;

    #[test]
    fn base() {
        let ctx = Navigator::new(SubwayStrategy::new());
        ctx.execute();

        let ctx = Navigator::new(RidingStrategy::new());
        ctx.execute();
    }
}