use super::implementor::DrawAPI;

pub struct RedCircle;

impl DrawAPI for RedCircle {
    fn draw_circle(&self, x: i32, y: i32, radius: i32) {
        println!("draw circle[ color: red, x: {}, y: {}, radius: {} ]", x, y, radius);
    }
}

impl RedCircle {
    pub fn new() -> Self {
        RedCircle
    }
}

pub struct GreenCircle;

impl DrawAPI for GreenCircle {
    fn draw_circle(&self, x: i32, y: i32, radius: i32) {
        println!("draw circle[ color: green, x: {}, y: {}, radius: {} ]", x, y, radius);
    }
}

impl GreenCircle {
    pub fn new() -> Self {
        GreenCircle
    }
}