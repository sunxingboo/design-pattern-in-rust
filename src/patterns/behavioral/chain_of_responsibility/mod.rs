mod middleware;
mod middleware_auth;
mod middleware_update;


#[cfg(test)]
mod tests {
    use crate::patterns::behavioral::chain_of_responsibility::middleware::Middleware;
    use super::middleware::{Request};
    use super::middleware_auth::MiddlewareAuth;
    use super::middleware_update::MiddlewareUpdate;

    #[test]
    fn base() {
        let r = Request::new("test user".to_string());
        let mut h1 = MiddlewareAuth::new();
        let h2 = MiddlewareUpdate::new();

        h1.set_next(Box::new(h2));
        assert_eq!(Result::Ok(true), h1.handle(r));

        let r = Request::new("auth failure test".to_string());
        assert_eq!(Result::Err("auth failed."), h1.handle(r));

        let r = Request::new("update failure test".to_string());
        assert_eq!(Result::Err("update failed."), h1.handle(r));
    }
}