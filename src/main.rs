use flintapi::{json, App};
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/api/status", |_req| async {
        json(json!({
            "status": "ok",
            "message": "FlintAPI is live!"
        }))
    });

    app.run("127.0.0.1:8000").await;
}
