/// 插座。
pub trait Outlet {
    fn socket_shape(&self);
    fn voltage(&self);
}

/// 国标插座。与英标插头接口不兼容。
pub struct NationalStandardOutlet;

impl Outlet for NationalStandardOutlet {
    fn socket_shape(&self) {
        println!("socket shape: national standard.");
    }

    fn voltage(&self) {
        println!("voltage: 220V");
    }
}

impl NationalStandardOutlet {
    pub fn new() -> Self {
        NationalStandardOutlet
    }
}