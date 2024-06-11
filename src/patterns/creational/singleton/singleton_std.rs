use std::sync::{Arc, Mutex};

static mut SINGLETON: Option<Arc<Mutex<SingletonStd>>> = None;

pub struct SingletonStd;

impl SingletonStd {
    pub unsafe fn new() -> Arc<Mutex<SingletonStd>> {
        unsafe {
            SINGLETON
                .get_or_insert_with(
                    || Arc::new(
                        Mutex::new(
                            SingletonStd
                        ),
                    ),
                )
                .clone()
        }
    }
}