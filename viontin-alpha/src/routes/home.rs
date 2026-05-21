use viontin::{Request, Response};

pub fn index(_req: Request) -> Response {
    Response::html(include_str!("../../html/index.html"))
}

pub fn features(_req: Request) -> Response {
    Response::html(include_str!("../../html/features.html"))
}

pub fn about(_req: Request) -> Response {
    Response::html(include_str!("../../html/about.html"))
}

pub fn contact(_req: Request) -> Response {
    Response::html(include_str!("../../html/contact.html"))
}

pub fn blog(_req: Request) -> Response {
    Response::html(include_str!("../../html/blog.html"))
}
