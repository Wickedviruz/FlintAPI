use std::collections::HashMap;

use crate::handler::Handler;
use crate::request::Request;

pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add(&mut self, method: &str, path: &str, handler: Handler) {
        let key = Self::route_key(method, path);
        self.routes.insert(key, handler);
    }

    pub async fn handle(&self, request: Request) -> String {
        let key = Self::route_key(&request.method, &request.path);
        if let Some(handler) = self.routes.get(&key) {
            handler(request).await
        } else {
            Self::not_found()
        }
    }

    fn route_key(method: &str, path: &str) -> String {
        format!("{} {}", method.to_uppercase(), path)
    }

    fn not_found() -> String {
        let body = r#"{"error": "Not Found"}"#;
        format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
    }
}
