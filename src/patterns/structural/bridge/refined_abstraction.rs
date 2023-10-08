use super::abstraction::Shape;
use super::implementor::DrawAPI;

pub struct Circle {
    x: i32,
    y: i32,
    radius: i32,
    draw_api: Box<dyn DrawAPI>,
}

impl Shape for Circle {
    fn draw(&self) {
        self.draw_api.draw_circle(self.x, self.y, self.radius);
    }
}

impl Circle {
    // 通过构造函数注入draw_api，完成桥接
    pub fn new(x: i32, y: i32, radius: i32, draw_api: Box<dyn DrawAPI>) -> Self {
        Circle {
            x,
            y,
            radius,
            draw_api,
        }
    }

    // 针对此图形定制各类操作......
}