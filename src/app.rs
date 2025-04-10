use crate::router::Router;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct App {
    router: Router,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn get<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(crate::request::Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = String> + Send + 'static,
    {
        self.router
            .add("GET", path, Box::new(move |req| Box::pin(handler(req))));
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
                let mut buffer = [0u8; 1024];
                let _ = socket.read(&mut buffer).await;

                let request_str = String::from_utf8_lossy(&buffer[..]).to_string();
                let request = crate::request::Request::from_raw(&request_str);

                let response = router.handle(request).await;

                let _ = socket.write_all(response.as_bytes()).await;
            });
        }
    }
}
