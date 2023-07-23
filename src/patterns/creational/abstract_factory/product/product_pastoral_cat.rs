use super::base::PastoralCat;

// 橘猫
#[derive(Default)]
pub struct GingerCat {
    name: String
}

impl GingerCat {
    pub fn new(name: String) -> Self {
        GingerCat{
            name
        }
    }
}

impl PastoralCat for GingerCat {
    fn say(&self) -> String {
        format!("cat: I'm a giner cat, my name is {}", self.name)
    }
}

// 三花
#[derive(Default)]
pub struct CalicoCat {
    name: String
}

impl CalicoCat {
    pub fn new(name: String) -> Self {
        CalicoCat{
            name
        }
    }
}

impl PastoralCat for CalicoCat {
    fn say(&self) -> String {
        format!("cat: I'm a calico cat, my name is {}", self.name)
    }
}