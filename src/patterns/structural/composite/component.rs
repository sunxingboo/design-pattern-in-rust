/// 组件。组成系统结构的基本单元。
pub trait Component {
    fn open(&self);
    fn show(&self, deep: i32);
}

