use flintapi::{json, App};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/api/users", |_req| async {
        json(json!({ "message": "GET revied!" }))
    });

    app.post("/api/users", |_req| async move {
        println!("Recived in rwa body {}", _req.body);
        match _req.json::<User>() {
            Ok(user) => json(json!({ "user": user})),
            Err(e) => {
                println!("JSON error: {:?}", e);
                json(json!({ "error": "Invalid JSON"}))
            }
        }
    });

    app.put("/api/users/1", |_req| async {
        json(json!({ "message": "PUT recived!"}))
    });

    app.delete("/api/users/1", |_req| async {
        json(json!({ "message": "DELETE recived!"}))
    });

    app.run("127.0.0.1:8000").await;
}
