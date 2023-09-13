mod command;
mod receiver;
mod invoker;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::receiver::MotherBoard;
    use super::invoker::Chassis;
    use super::command::{CommandStart, CommandShutdown, CommandRestart};

    #[test]
    fn base() {
        let mother_board = Rc::new(
            RefCell::new(MotherBoard::new()),
        );

        let start_command = CommandStart::new(mother_board.clone());
        let shutdown_command = CommandShutdown::new(mother_board.clone());
        let restart_command = CommandRestart::new(mother_board.clone());

        let c = Chassis::new(
            Box::new(start_command),
            Box::new(shutdown_command),
            Box::new(restart_command),
        );

        c.press_button_1();
        c.press_button_2();
        c.press_button_3();
    }
}