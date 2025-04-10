use serde::Serialize;

/// Returnerar en JSON-sträng med HTTP 200-svar
pub fn json<T: Serialize>(data: T) -> String {
    let body = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}
