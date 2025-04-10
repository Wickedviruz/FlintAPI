use std::future::Future;
use std::pin::Pin;

use crate::request::Request;

// Definiera Handler-typen
pub type Handler =
    Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync + 'static>;
