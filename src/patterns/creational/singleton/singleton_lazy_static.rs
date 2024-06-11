use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref INSTANCE: Arc<Mutex<SingletonLazyStatic>> = Arc::new(
        Mutex::new(
            SingletonLazyStatic::init()
        )
    );
}

pub struct SingletonLazyStatic;

#[allow(unused)]
impl SingletonLazyStatic {
    fn init() -> Self {
        SingletonLazyStatic
    }

    pub fn new() -> Arc<Mutex<SingletonLazyStatic>> {
        INSTANCE.clone()
    }
}
