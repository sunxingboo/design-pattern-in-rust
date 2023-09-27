mod proxy;
mod subject;

#[cfg(test)]
mod tests {
    use super::proxy::ProxyAccessor;
    use super::subject::{GoogleAccessor, Accessor};

    #[test]
    fn base() {
        let p = ProxyAccessor::new(
            Box::new(GoogleAccessor::new()),
        );

        p.execute();
    }
}
