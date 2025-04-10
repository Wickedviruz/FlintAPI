#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn from_raw(raw: &str) -> Self {
        let mut lines = raw.lines();
        let first_line = lines.next().unwrap_or_default();

        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("/").to_string();

        Request { method, path }
    }
}
