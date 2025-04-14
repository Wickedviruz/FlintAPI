use std::collections::HashMap;

/// Matchar ex: "/users/42" mot "/users/:id"
/// Om match: returnerar params {"id": "42"}
pub fn match_path(route: &str, actual: &str) -> Option<HashMap<String, String>> {
    let route_parts: Vec<_> = route.trim_matches('/').split('/').collect();
    let actual_parts: Vec<_> = actual.trim_matches('/').split('/').collect();

    if route_parts.len() != actual_parts.len() {
        return None;
    }

    let mut params = HashMap::new();

    for (route_part, actual_part) in route_parts.iter().zip(actual_parts.iter()) {
        if route_part.starts_with(":") {
            params.insert(route_part[1..].to_string(), actual_part.to_string());
        } else if route_part != actual_part {
            return None;
        }
    }

    Some(params)
}
