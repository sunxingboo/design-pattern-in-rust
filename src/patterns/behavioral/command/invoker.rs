use super::command::Command;

/// 机箱；命令的调用者，不同的按钮包装了不同的命令；
pub(crate) struct Chassis {
    button1: Box<dyn Command>,
    button2: Box<dyn Command>,
    button3: Box<dyn Command>,
}

impl Chassis {
    pub fn new(b1: Box<dyn Command>, b2: Box<dyn Command>, b3: Box<dyn Command>) -> Self {
        Chassis {
            button1: b1,
            button2: b2,
            button3: b3,
        }
    }

    // pub fn set_command_for_button_1(&mut self, cmd: Box<dyn Command>) -> &Self {
    //     self.button1 = cmd;
    //     self
    // }
    //
    // pub fn set_command_for_button_2(&mut self, cmd: Box<dyn Command>) -> &Self {
    //     self.button2 = cmd;
    //     self
    // }
    //
    // pub fn set_command_for_button_3(&mut self, cmd: Box<dyn Command>) -> &Self {
    //     self.button3 = cmd;
    //     self
    // }

    pub fn press_button_1(&self) {
        self.button1.execute();
    }

    pub fn press_button_2(&self) {
        self.button2.execute();
    }

    pub fn press_button_3(&self) {
        self.button3.execute();
    }
}