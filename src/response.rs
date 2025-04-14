use serde::Serialize;

pub fn cors_preflight() -> String {
    let headers = [
        "HTTP/1.1 204 No Content",
        "Access-Control-Allow-Origin: *",
        "Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS",
        "Access-Control-Allow-Headers: Content-Type",
        "Content-Length: 0",
        "\r\n",
    ];

    headers.join("\r\n")
}


pub fn with_cors(response: &str) -> String {
    format!(
        "{}Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\n",
        response.trim_end()
    )
}

/// Returnerar en JSON-sträng med HTTP 200-svar
pub fn json<T: Serialize>(data: T) -> String {
    let body = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}
