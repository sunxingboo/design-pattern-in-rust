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
        let h1 = MiddlewareAuth::new();
        let h2 = MiddlewareUpdate::new();

        h1.set_next(Box::new(h2))
    }
}
