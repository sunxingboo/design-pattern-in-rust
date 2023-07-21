use super::factory::Animal;

#[derive(Default)]
pub struct Cat {
    name: String,
}

// 构造方法
impl Cat {
    pub fn new() -> Self {
        Cat {name: "mimi".to_string()}
    }
}

// 实现工厂产品接口
impl Animal for Cat {
    fn say(&self) -> String {
        format!("I'm a cat, my name is {}", self.name)
    }
}