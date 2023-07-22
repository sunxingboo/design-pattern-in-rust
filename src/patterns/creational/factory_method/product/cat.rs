use super::base::Animal;

#[derive(Default)]
pub struct Cat {
    name: String,
}

// 实现构造方法
impl Cat {
    pub fn new() -> Self {
        Cat {name: "mimi".to_string()}
    }
}

// 实现工厂抽象产品接口
impl Animal for Cat {
    fn say(&self) -> String {
        format!("I'm a cat, my name is {}", self.name)
    }
}