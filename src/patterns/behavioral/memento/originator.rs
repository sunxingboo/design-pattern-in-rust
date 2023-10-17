use super::memento::Memento;

/// 原发器接口。
pub trait Originator {
    fn set(&mut self, _: String);
    fn save(&self) -> Box<dyn Memento>;
    fn restore(&mut self, _: &dyn Memento);
}