
// 原型接口
pub trait Prototype {
    type AssociateType;

    fn clone(&self) -> Self::AssociateType;
}