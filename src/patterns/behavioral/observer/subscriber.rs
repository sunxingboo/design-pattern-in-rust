
pub trait Subscriber {
    fn receive(&self, msg: i32);
}

/// ConsumerA
pub struct ConsumerA;

impl ConsumerA {
    pub fn new() -> Self {
        ConsumerA{}
    }
}

impl Subscriber for ConsumerA {
    fn receive(&self, msg: i32) {
        println!("ConsumerA received: {}", msg*2);
    }
}

/// ConsumerB
pub struct ConsumerB;

impl ConsumerB {
    pub fn new() -> Self {
        ConsumerB{}
    }
}

impl Subscriber for ConsumerB {
    fn receive(&self, msg: i32) {
        println!("ConsumerB received: {}", msg*10);
    }
}