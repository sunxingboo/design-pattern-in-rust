use super::base::Animal;

#[derive(Default)]
pub struct Dog {
    name: String,
}

// 实现构造方法
impl Dog {
    pub fn new() -> Self {
        Dog{name: "wangwang".to_string()}
    }
}

// 实现工厂抽象产品接口
impl Animal for Dog {
    fn say(&self) -> String {
        format!("I'm a dog, my name is {}", self.name)
    }
}