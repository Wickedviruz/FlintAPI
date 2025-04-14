use std::collections::HashMap;

/// Parsar headers till HashMap
pub fn parse_headers(raw: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    for line in raw.lines().skip(1) { // skippar första raden (GET /path HTTP/1.1)
        if let Some((key, value)) = line.split_once(":") {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    headers
}