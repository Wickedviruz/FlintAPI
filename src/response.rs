use std::collections::HashMap;

use serde::Serialize;

pub struct  Response {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String,String>,
}

impl Response {
    pub fn ok<T: ToString>(body: T) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".into(), "application/json".into());

        let body_str = body.to_string();
        headers.insert("Content-Length".into(), body_str.len().to_string());

        Self {
            status_code: 200,
            body: body_str,
            headers,
        }
    }

    pub fn bad_request<T: ToString>(body: T) -> Self {
        let mut resp = Self::ok(body);
        resp.status_code = 400;
        resp
    }

    pub fn not_found() -> Self {
        Self::bad_request(r#"{"error":"Not Found"}"#)
    }

    pub fn with_cors(mut self) -> Self {
        self.headers.insert("Access-Control-Allow-Origin".into(), "*".into());
        self.headers.insert("Access-Control-Allow-Methods".into(), "GET, POST, PUT, DELETE, OPTIONS".into());
        self.headers.insert("Access-Control-Allow-Headers".into(), "Content-Type".into());
        self
    }

    pub fn cors_preflight() -> Response {
        let mut headers = HashMap::new();
        headers.insert("Access-Control-Allow-Origin".into(), "*".into());
        headers.insert("Access-Control-Allow-Methods".into(), "GET, POST, PUT, DELETE, OPTIONS".into());
        headers.insert("Access-Control-Allow-Headers".into(), "Content-Type".into());
        headers.insert("Content-Length".into(), "0".into());
    
        Response {
            status_code: 204,
            body: String::new(),
            headers,
        }
    }

    pub fn json<T: Serialize>(data: T) -> Self {
        let body = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());
    
        let mut headers = HashMap::new();
        headers.insert("Content-Type".into(), "application/json".into());
        headers.insert("Content-Length".into(), body.len().to_string());
    
        Self {
            status_code: 200,
            body,
            headers,
        }
    }

    pub fn to_http_string(&self) -> String {
        let mut resp = format!("HTTP/1.1 {} {}\r\n", self.status_code, status_text(self.status_code));

        for (k, v) in &self.headers {
            resp.push_str(&format!("{}: {}\r\n", k, v));
        }

        resp.push_str("\r\n");
        resp.push_str(&self.body);
        resp
    }
}

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}