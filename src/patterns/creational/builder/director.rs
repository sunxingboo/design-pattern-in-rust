use crate::patterns::creational::builder::builder::base::Builder;

// 引导器
pub struct Director<T: Builder> {
    builder: T,
}

impl <T: Builder> Director<T> {
    // 引导器构造函数
    pub fn new(builder: T) -> Self {
        Director {
            builder,
        }
    }

    // 获取当前的生成器实例
    pub fn get_builder(&self) -> &T {
        &self.builder
    }

    // 构建
    pub fn build(&mut self) {
        self.builder.set_appearance();
        self.builder.set_attribute();
    }
}