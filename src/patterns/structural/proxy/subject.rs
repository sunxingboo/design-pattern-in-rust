/// 主题特征。
pub(crate) trait Subject {
    fn execute(&self);
}
/// 谷歌访问器。实际执行请求。
pub(crate) struct GoogleAccessor;

impl Subject for GoogleAccessor {
    fn execute(&self) {
        println!("proxy: access to google.");
    }
}

impl GoogleAccessor {
    pub(crate) fn new() -> Self {
        GoogleAccessor
    }
}