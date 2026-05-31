use viontin::{Request, Response};
use viontin::macros::get;

#[get("/")]
pub fn index(_req: Request) -> Response { Response::html(include_str!("../../html/index.html")) }

#[get("/features")]
pub fn features(_req: Request) -> Response { Response::html(include_str!("../../html/features.html")) }

#[get("/ecosystems")]
pub fn ecosystems(_req: Request) -> Response { Response::html(include_str!("../../html/ecosystems.html")) }
