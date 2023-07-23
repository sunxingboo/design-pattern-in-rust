use super::base::BritishShorthairCat;

// 蓝白
#[derive(Default)]
pub struct BlueWhiteCat {
    name: String,
}

impl BlueWhiteCat {
    pub fn new(name: String) -> Self {
        BlueWhiteCat{
            name,
        }
    }
}

impl BritishShorthairCat for BlueWhiteCat {
    fn say(&self) -> String {
        format!("cat: I'm a bule white cat, my name is {}.", self.name)
    }
}

// 金渐层
#[derive(Default)]
pub struct FelinaeCat {
    name: String,
}

impl FelinaeCat {
    pub fn new(name: String) -> Self {
        FelinaeCat{
            name,
        }
    }
}

impl BritishShorthairCat for FelinaeCat {
    fn say(&self) -> String {
        format!("cat: I'm a felinaeCat, my name is {}.", self.name)
    }
}