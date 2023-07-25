
// 生成器接口
pub trait Builder {
    type AssociateType;

    fn set_appearance(&mut self);
    fn set_attribute(&mut self);
}