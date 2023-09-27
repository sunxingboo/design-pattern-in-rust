/// 主题特征。
pub(crate) trait Accessor {
    fn execute(&self);
}
/// 谷歌访问器。实际执行请求。
pub(crate) struct GoogleAccessor;

impl Accessor for GoogleAccessor {
    fn execute(&self) {
        println!("proxy: access to google.");
    }
}

impl GoogleAccessor {
    pub(crate) fn new() -> Self {
        GoogleAccessor
    }
}