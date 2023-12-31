use super::subject::Accessor;

/// 代理访问器
pub struct ProxyAccessor {
    real: Box<dyn Accessor>
}

impl Accessor for ProxyAccessor {
    fn execute(&self) {
        println!("log: before access.");
        self.real.execute();
        println!("log: after access.");
    }
}

impl ProxyAccessor {
    // 可以结合工厂模式生成持有的real对象。这里简单表述不举例实现了。
    pub fn new(real: Box<dyn Accessor>) -> Self {
        ProxyAccessor {
            real,
        }
    }
}