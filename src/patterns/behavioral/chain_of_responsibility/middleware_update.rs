use super::middleware::{Middleware, Request};

// MiddlewareAuth 处理权限认证的中间件
pub struct MiddlewareUpdate {
    next: Option<Box<dyn Middleware>>
}

impl MiddlewareUpdate {
    pub fn new() -> Self {
        MiddlewareUpdate{
            next: None,
        }
    }
}

// 实现中间件接口
impl Middleware for MiddlewareUpdate {
    fn set_next(&mut self, m: Box<dyn Middleware>) {
        self.next = Some(m)
    }

    fn handle(&self, r: Request) -> Result<(), &'static str> {
        format!("update success: {}", r.get_name());

        match &self.next {
            Some(next) => {
                next.handle(r)
            },
            None => {

            }
        }
    }
}