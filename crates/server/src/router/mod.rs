use std::collections::HashMap;
use std::sync::Arc;

use crate::http::HttpMethod;
use crate::request::Request;
use crate::response::Responsable;

pub type Handler<T: Responsable> = Arc<dyn Fn(T) + Send + Sync>;

// pub type Handler<T> = fn() -> Option<T>;
// pub enum Handler<T> {
//     A(fn() -> T),
//     B(fn() -> Option<T>),
//     Fn(fn() -> Vec<T>),
// }

pub struct Router<T> {
    routes: Vec<String>,
    routes_map: HashMap<(HttpMethod, String), Handler<T>>,
}

impl<T> Router<T> {
    pub fn new() -> Router<T> {
        Router {
            routes: vec![],
            routes_map: HashMap::new(),
        }
    }

    fn register(&mut self, method: HttpMethod, path: &str, handler: Handler<T>) {
        self.routes_map.insert((method, path.to_string()), handler);
    }

    pub fn get(&mut self, path: &str, handler: Handler<T>) {
        self.register(HttpMethod::GET, path, handler)
    }

    pub fn find(&mut self, method: HttpMethod, path: &str) -> Option<&Handler<T>> {
        return self.routes_map.get(&(method, path.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use crate::{http::HttpMethod, response::Responsable};

    use super::Router;

    struct Test {}

    impl Responsable for Test {}

    // #[test]
    // fn new() {
    //     let mut r: Router<_> = Router::new();
    //     assert_eq!(r.routes.len(), 0);
    // }

    #[test]
    fn register() {
        fn some_func() -> Test {
            Test {}
        }

        let mut r: Router<Test> = Router::new();
        assert_eq!(r.routes.len(), 0);

        r.register(HttpMethod::GET, "/users", some_func);
        assert_eq!(r.routes.len(), 1);
    }
}
