use super::subscriber::Subscriber;

/// 消息队列。观察者角色，当有新消息到达时，将其推送给每一个订阅者。
pub struct MessageQueue {
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            subscribers: vec![],
        }
    }

    pub fn add_subscriber(&mut self, s: Box<dyn Subscriber>) -> &mut Self {
        self.subscribers.push(s);
        self
    }

    /// 观察者模式的整体结构与中介者模式很相似，但请求调度却大不相同。
    /// 中介者模式中请求路径为：成员->中介者->其他成员，中介者做了一些协调工作。
    /// 而在观察者模式中的成员为被动接收请求：发布者->订阅者。
    pub fn notify(&self, number: i32) {
        for i in &self.subscribers {
            i.receive(number);
        }
    }
}
