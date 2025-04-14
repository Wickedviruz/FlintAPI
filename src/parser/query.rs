use std::collections::HashMap;

/// Parsar querysträngar (?key=val&x=1) till HashMap
pub fn parse_query(query: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for pair in query.split('&') {
        let mut split = pair.splitn(2, '=');
        let key = split.next().unwrap_or("");
        let value = split.next().unwrap_or("");

        if !key.is_empty() {
            result.insert(key.to_string(), value.to_string());
        }
    }

    result
}