use crate::handler::Handler;
use crate::parser::path::match_path;
use crate::request::Request;

pub struct Router {
    routes: Vec<(String, String, Handler)>, // (method, path, handler)
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add(&mut self, method: &str, path: &str, handler: Handler) {
        self.routes.push((method.to_string(), path.to_string(), handler));
    }

    pub async fn handle(&self, mut request: Request) -> String {
        if request.method == "OPTIONS" {
            return crate::response::cors_preflight(); // catch all OPTIONS!
        }
    
        for (method, route_path, handler) in &self.routes {
            if &request.method == method {
                if let Some(params) = match_path(route_path, &request.path) {
                    request.params = params;
                    let raw = handler(request).await;
                    return crate::response::with_cors(&raw);
                }
            }
        }
    
        crate::response::with_cors(&Self::not_found())
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