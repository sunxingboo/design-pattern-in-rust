use std::cell::RefCell;
use std::rc::Rc;
use super::receiver::MotherBoard;

/// 命令，被调用者传递给接受者执行
pub trait Command {
    fn execute(&self);
}

/// 启动命令
pub struct CommandStart {
    mother_board: Rc<RefCell<MotherBoard>>,
}

impl CommandStart {
    pub fn new(m: Rc<RefCell<MotherBoard>>) -> Self {
        CommandStart {
            mother_board: m,
        }
    }
}

impl Command for CommandStart {
    fn execute(&self) {
        self.mother_board.borrow().start();
    }
}

/// 关机命令
pub struct CommandShutdown {
    mother_board: Rc<RefCell<MotherBoard>>,
}

impl CommandShutdown {
    pub fn new(m: Rc<RefCell<MotherBoard>>) -> Self {
        CommandShutdown {
            mother_board: m,
        }
    }
}

impl Command for CommandShutdown {
    fn execute(&self) {
        self.mother_board.borrow().shutdown();
    }
}

/// 重启命令
pub struct CommandRestart {
    mother_board: Rc<RefCell<MotherBoard>>,
}

impl CommandRestart {
    pub fn new(m: Rc<RefCell<MotherBoard>>) -> Self {
        CommandRestart {
            mother_board: m,
        }
    }
}

impl Command for CommandRestart {
    fn execute(&self) {
        self.mother_board.borrow().restart();
    }
}