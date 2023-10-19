use super::strategy::RouteStrategy;

/// 地铁路径生成策略
pub struct SubwayStrategy;

impl RouteStrategy for SubwayStrategy {
    fn build(&self) {
        println!("subway: S -> A1 -> B1 -> C1 -> E");
    }
}

impl SubwayStrategy {
    pub fn new() -> SubwayStrategy {
        SubwayStrategy
    }
}

/// 骑行路径生成策略
pub struct RidingStrategy;

impl RouteStrategy for RidingStrategy {
    fn build(&self) {
        println!("riding: S -> A2 -> B1 -> E");
    }
}

impl RidingStrategy {
    pub fn new() -> RidingStrategy {
        RidingStrategy
    }
}