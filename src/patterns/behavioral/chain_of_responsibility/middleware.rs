// Middleware 中间件接口
pub trait Middleware {
    fn set_next(&self, m: Box<dyn Middleware>);
    fn handle(&self, r: Request) -> Result<(), &'static str>;
}

// Request 请求数据
pub struct Request {
    username: String,
}

impl Request {
    pub fn new(name: String) -> Self {
        Request{
            username: name,
        }
    }

    pub fn get_name(&self) -> String {
        self.username.clone()
    }
}