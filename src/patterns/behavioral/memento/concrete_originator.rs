use std::rc::Rc;
use super::memento::Memento;
use super::originator::Originator;
use super::concrete_memento::Snapshot;

/// 具体原发器。支持生成快照和数据恢复。
pub struct Editor {
    text: String,
}

impl Originator for Editor {
    fn set(&mut self, text: String) {
        self.text = text;
    }

    fn save(&self) -> Box<dyn Memento> {
        Box::new(Snapshot::new(self.text.clone()))
    }

    fn restore(&mut self, snapshot: &dyn Memento) {
        self.text = snapshot.get_text();
    }
}

impl Editor {
    pub fn new() -> Self {
        Editor{
            text: String::new(),
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}
