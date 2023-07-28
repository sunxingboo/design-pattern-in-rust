use std::sync::{Arc, Mutex};

struct SingletonCat {
    instance: Arc<Mutex<Cat>>,
}

impl SingletonCat {
    pub fn new() -> Self {
        let cat = Cat{
            name: "cat_king".to_string(),
            role: "admin".to_string(),
        };

        let instance = Arc::new(Mutex::new(cat));

        SingletonCat{
            instance,
        }
    }

}

#[derive(Default)]
struct Cat {
    name: String,
    role: String
}