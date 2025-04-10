#FlintAPI

FlintAPI Is a minimal, lightweight webframework for Rust, inspired by FastAPI.
the goal is to offer a supersimple syntax to build fast and clear APIs.

## GOAL:
- Easy routing: 'GET', 'POST', 'PUT', 'DELTE', etc.
- JSON-support
- Async-based on 'Tokio'
- Build from the ground up with rust.

## Example
```rust
#[get("ping")]
async fn ping -> impl Responder {
    json!({"pong": true})
}
```

## Installation (when publiced)
```toml
[dependencies]
flinapi = "0.1"
```

## Under development...
Contributions, ideas nad PRs are welcome!
