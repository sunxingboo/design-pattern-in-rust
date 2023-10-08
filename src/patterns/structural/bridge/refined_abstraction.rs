use super::abstraction::Shape;
use super::implementor::Color;

pub struct Circle {
    x: i32,
    y: i32,
    radius: i32,
    color: Box<dyn Color>,
}

impl Shape for Circle {
    fn draw(&self) {
        print!("draw circle[ x: {}, y: {}, radius: {} ], ", self.x, self.y, self.radius);
        self.color.dyeing();
    }
}

impl Circle {
    /// 通过构造函数注入draw_api，完成桥接
    /// 与颜色相关的操作均委托给dyeing
    pub fn new(x: i32, y: i32, radius: i32, color: Box<dyn Color>) -> Self {
        Circle {
            x,
            y,
            radius,
            color,
        }
    }

    // 针对此图形定制各类操作......
}