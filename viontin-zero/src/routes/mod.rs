use std::sync::Arc;
use viontin::{Router, Request, Response};

pub fn index(_req: Request) -> Response { Response::html(include_str!("../../html/index.html")) }
pub fn features(_req: Request) -> Response { Response::html(include_str!("../../html/features.html")) }
pub fn ecosystems(_req: Request) -> Response { Response::html(include_str!("../../html/ecosystems.html")) }

pub fn register(router: Router) -> Router {
    let h = |f: fn(Request) -> Response| -> Arc<dyn Fn(Request) -> Response + Send + Sync> { Arc::new(f) };
    router
        .get("/",          h(index))
        .get("/features",  h(features))
        .get("/ecosystems", h(ecosystems))
}
