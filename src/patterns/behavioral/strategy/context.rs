use super::strategy::RouteStrategy;

pub struct Navigator<T: RouteStrategy> {
    strategy: T,
}

impl<T: RouteStrategy> Navigator<T> {
    pub fn new(strategy: T) -> Navigator<T> {
        Navigator {
            strategy
        }
    }

    #[allow(unused)]
    pub fn change_strategy(&mut self, strategy: T) {
        self.strategy = strategy;
    }

    pub fn execute(&self) {
        self.strategy.build();
    }
}