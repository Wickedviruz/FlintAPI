use std::collections::HashMap;
use crate::parser::{headers, query};

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl Request {
    pub fn from_raw(raw: &str) -> Self {
        let (head, body) = raw.split_once("\r\n\r\n").unwrap_or((raw, ""));
        let mut lines = head.lines();
        let first_line = lines.next().unwrap_or_default();
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let full_path = parts.next().unwrap_or("/").to_string();

        // Delar upp path och query
        let (path, query_str) = full_path.split_once('?').unwrap_or((full_path.as_str(), ""));

        let headers = headers::parse_headers(head);
        let query = query::parse_query(query_str);

        Self {
            method,
            path: path.to_string(),
            body: body.to_string(),
            headers,
            params: HashMap::new(),
            query,
        }
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }
}