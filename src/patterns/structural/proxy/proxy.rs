use super::subject::Subject;

/// 通用抽象访问器
pub(crate) struct ProxyAccessor {
    real: Box<dyn Subject>
}

impl Subject for ProxyAccessor {
    fn execute(&self) {
        println!("log: before access.");
        self.real.execute();
        println!("log: after access.");
    }
}

impl ProxyAccessor {
    // 可以结合工厂模式生成持有的real对象。这里简单表述不举例实现了。
    pub(crate) fn new(real: Box<dyn Subject>) -> Self {
        ProxyAccessor {
            real,
        }
    }
}