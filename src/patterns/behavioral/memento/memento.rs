/// 备忘录接口。
pub trait Memento {
    fn get_text(&self) -> String;
}