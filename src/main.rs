use flintapi::{App, json};
use flintapi::response::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get_json("/api/users", |_| {
        json!({ "message": "GET received!" })
    });

    app.get("/api/user/:id", |req| async move {
        let id = req.params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
        Response::json(json!({ "user_id": id }))
    });

    app.get("/api/search", |req| async move {
        let q = req.query_param("q");
        Response::json(json!({ "search": q }))
    });

    app.post_json("/api/users", |user: User| {
        json!({ "created": true, "user": user })
    });

    app.put("/api/users/1", |_| async {
        Response::json(json!({ "message": "PUT received!" }))
    });

    app.delete("/api/users/1", |_| async {
        Response::json(json!({ "message": "DELETE received!" }))
    });

    app.run("127.0.0.1:8000").await;
}
