use super::state::State;
use super::concrete_state::Draft;

pub struct Document {
    author: String,
    state: Box<dyn State>,
}

impl Document {
    /// 新建文档默认为草稿状态
    pub fn new(author: String) -> Self {
        Document {
            author,
            state: Box::new(Draft::new()),
        }
    }

    pub fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }

    pub fn author_is_admin(&self) -> bool {
        match self.author.as_str() {
            "admin" => true,
            _ => false
        }
    }

    pub fn publish(&mut self) -> Box<dyn State> {
        self.state.handle(self)
    }
}