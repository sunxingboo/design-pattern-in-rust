/// 主板；命令的接收者，真正执行命令的角色；
pub struct MotherBoard;

impl MotherBoard {
    pub fn new() -> Self {
        MotherBoard{}
    }

    pub fn start(&self) {
        println!("start-command executing...\nstarted.\n");
    }

    pub fn shutdown(&self) {
        println!("shutdown-command executing...\nstopped.\n");
    }

    pub fn restart(&self) {
        println!("restart-command executing...\n");

        self.shutdown();
        self.start();

        println!("restarted.");
    }
}
