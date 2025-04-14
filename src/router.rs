use crate::handler::Handler;
use crate::parser::path::match_path;
use crate::request::Request;
use crate::response::Response;

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

    pub async fn handle(&self, mut request: Request) -> Response {
        if request.method == "OPTIONS" {
            return Response::cors_preflight();
        }
    
        for (method, route_path, handler) in &self.routes {
            if &request.method == method {
                if let Some(params) = match_path(route_path, &request.path) {
                    request.params = params;
                    let raw = handler(request).await; // raw är String
    
                    return raw.with_cors(); // wrappa det i ett Response
                }
            }
        }
    
        Response::not_found().with_cors() // fallback
    }
}