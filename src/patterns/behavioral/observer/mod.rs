mod publisher;
mod subscriber;

#[cfg(test)]
mod tests {
    use super::publisher::MessageQueue;
    use super::subscriber::{ConsumerA, ConsumerB};

    #[test]
    fn base() {
        let mut p = MessageQueue::new();
        let s1 = ConsumerA::new();
        let s2 = ConsumerB::new();

        p.add_subscriber(Box::new(s1));
        p.add_subscriber(Box::new(s2));

        p.notify(7);
    }
}