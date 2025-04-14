use crate::router::Router;
use crate::json;
use crate::request::Request;
use crate::response::Response;

use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024; // 10MB

pub struct App {
    router: Router,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn get_json<T, F>(&mut self, path: &str, handler: F)
        where
            T: serde::Serialize + 'static,
            F: Fn(Request) -> T + Send + Sync + 'static,
        {
            let handler = Arc::new(handler); // dela referens

            self.get(path, move |req| {
                let handler = Arc::clone(&handler); // klona in
                async move {
                    Response::json(handler(req))
                }
            });
        }
    
    pub fn post_json<T, F, R>(&mut self, path: &str, handler: F)
        where
            T: serde::de::DeserializeOwned + 'static,
            R: serde::Serialize + 'static,
            F: Fn(T) -> R + Send + Sync + 'static,
        {
            use std::sync::Arc;
            let handler = Arc::new(handler);
        
            self.post(path, move |req| {
                let handler = Arc::clone(&handler);
                async move {
                    match req.json::<T>() {
                        Ok(data) => Response::json(handler(data)),
                        Err(_) => Response::bad_request(json!({ "error": "Invalid JSON" })),
                    }
                }
            });
        }

    pub fn get<F, Fut>(&mut self, path: &str, handler: F)
        where
            F: Fn(crate::request::Request) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future<Output = crate::response::Response> + Send + 'static,
        {
            self.router
                .add("GET", path, Box::new(move |req| Box::pin(handler(req))));
        }

    pub fn post<F, Fut>(&mut self, path: &str, handler: F)
        where
            F: Fn(crate::request::Request) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future<Output = crate::response::Response> + Send + 'static,
        {
            self.router
                .add("POST", path, Box::new(move |req| Box::pin(handler(req))));
        }

    pub fn put<F, Fut>(&mut self, path: &str, handler: F)
        where
            F: Fn(crate::request::Request) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future<Output = crate::response::Response> + Send + 'static,
        {
            self.router
                .add("PUT", path, Box::new(move |req| Box::pin(handler(req))));
        }

    pub fn delete<F, Fut>(&mut self, path: &str, handler: F)
        where
            F: Fn(crate::request::Request) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future<Output = crate::response::Response> + Send + 'static,
        {
            self.router
                .add("DELETE", path, Box::new(move |req| Box::pin(handler(req))));
        }

    pub fn options<F, Fut>(&mut self, path: &str, handler: F)
        where
            F: Fn(crate::request::Request) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future<Output = crate::response::Response> + Send + 'static,
        {
            self.router
                .add("OPTIONS", path, Box::new(move |req| Box::pin(handler(req))));
        }

    pub async fn run(self, addr: &str) {
        use tokio::net::TcpListener;

        let listener = TcpListener::bind(addr).await.expect("Failed to bind");

        println!("FlintAPI running on http://{}", addr);

        let router = Arc::new(self.router);

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let router = Arc::clone(&router);

            tokio::spawn(async move {
                let mut buffer = [0u8; 4096];
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(_) => return,
                };
            
                // Parsar header
                let request_str = String::from_utf8_lossy(&buffer[..n]);
                let (headers_raw, _) = request_str.split_once("\r\n\r\n").unwrap_or((&request_str, ""));
                let content_length = headers_raw
                    .lines()
                    .find_map(|line| {
                        if line.to_lowercase().starts_with("content-length:") {
                            line.split(':').nth(1)?.trim().parse::<usize>().ok()
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);
            
                if content_length > MAX_REQUEST_SIZE {
                    let _ = socket.write_all(
                        b"HTTP/1.1 413 Payload Too Large\r\nContent-Length: 0\r\n\r\n"
                    ).await;
                    return;
                }
            
                // Läs hela requesten (inkl. body)
                let mut total_buffer = Vec::from(&buffer[..n]);
                while total_buffer.len() < content_length + headers_raw.len() + 4 {
                    let mut temp = [0u8; 4096];
                    let n = match socket.read(&mut temp).await {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => return,
                    };
                    total_buffer.extend_from_slice(&temp[..n]);
            
                    if total_buffer.len() > MAX_REQUEST_SIZE + 8192 {
                        let _ = socket.write_all(
                            b"HTTP/1.1 413 Payload Too Large\r\nContent-Length: 0\r\n\r\n"
                        ).await;
                        return;
                    }
                }
            
                // Nu har vi hela requesten
                let request_str = String::from_utf8_lossy(&total_buffer);
                let request = crate::request::Request::from_raw(&request_str);
            
                let response = router.handle(request).await;
                let raw = response.to_http_string();
                let _ = socket.write_all(raw.as_bytes()).await;
            });
        }
    }
}
