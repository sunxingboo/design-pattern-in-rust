use super::implementor::Color;

pub struct Red;

impl Color for Red {
    fn dyeing(&self) {
        println!("dyeing to red.");
    }
}

impl Red {
    pub fn new() -> Self {
        Red
    }
}

pub struct Green;

impl Color for Green {
    fn dyeing(&self) {
        println!("dyeing to green.");
    }
}

impl Green {
    pub fn new() -> Self {
        Green
    }
}