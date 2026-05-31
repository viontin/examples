use viontin::{Request, Response, StatusCode};
use viontin::macros::{get, post};

mod home;
mod blog;

#[get("/")]
fn index(_req: Request) -> Response { home::index(_req) }

#[get("/features")]
fn features_page(_req: Request) -> Response { home::features(_req) }

#[get("/about")]
fn about_page(_req: Request) -> Response { home::about(_req) }

#[get("/contact")]
fn contact_page(_req: Request) -> Response { home::contact(_req) }

#[post("/contact")]
fn do_contact(req: Request) -> Response {
    let body = req.body_str();
    let name = extract_form_field(body, "name").unwrap_or_else(|| "Anonymous".into());
    let email = extract_form_field(body, "email").unwrap_or_else(|| "unknown@example.com".into());
    Response::html(&format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Message Sent</title></head>
        <body><h1>Message Sent!</h1><p>Thanks, {}!</p><a href="/">Back</a></body></html>"#,
        name
    ))
}

#[get("/blog")]
fn blog_list(_req: Request) -> Response { home::blog(_req) }

#[get("/blog/:slug")]
fn blog_post(req: Request) -> Response { blog::post(req) }

#[get("/api/hello")]
fn api_hello(_req: Request) -> Response {
    Response::json(&serde_json::json!({"message": "Hello from Viontin!", "status": "ok"}))
}

#[get("/api/echo")]
fn api_echo(req: Request) -> Response {
    Response::json(&serde_json::json!({
        "method": "GET", "path": req.uri.path,
        "query": req.uri.query,
    }))
}

#[post("/api/echo")]
fn api_echo_post(req: Request) -> Response {
    Response::json(&serde_json::json!({
        "method": "POST",
        "body": req.body_str(),
        "content-type": req.header("content-type").unwrap_or(""),
    }))
}

#[get("/assets/tailwind.css")]
fn css_asset(_req: Request) -> Response {
    let path = std::path::Path::new("assets/tailwind.css");
    if path.exists() {
        match std::fs::read_to_string(path) {
            Ok(css) => Response::text(&css).header("content-type", "text/css; charset=utf-8"),
            Err(_) => Response::new(StatusCode::SERVER_ERROR).with_body("Failed to read CSS"),
        }
    } else {
        Response::new(StatusCode::NOT_FOUND).with_body("Run app to generate CSS")
    }
}

fn extract_form_field(body: &str, name: &str) -> Option<String> {
    for pair in body.split('&') {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next()?;
        let val = parts.next()?;
        if key == name {
            return Some(viontin::fw::support::url_decode(val));
        }
    }
    None
}
