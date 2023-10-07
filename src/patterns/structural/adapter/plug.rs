/// 插头。
pub trait Plug {
    fn pin_shape(&self);
    fn voltage(&self);
}

/// 港版插头（英标）。与国行插座互相不兼容。
pub struct BritishStandardPlug;

impl Plug for BritishStandardPlug {
    fn pin_shape(&self) {
        println!("shape: british standard.");
    }

    fn voltage(&self) {
        println!("voltage: 240V.")
    }
}

impl BritishStandardPlug {
    pub fn new() -> Self {
        BritishStandardPlug
    }
}