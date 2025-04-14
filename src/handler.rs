use std::future::Future;
use std::pin::Pin;

use crate::request::Request;
use crate::response::Response;

pub type Handler = Box<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync,
>;
