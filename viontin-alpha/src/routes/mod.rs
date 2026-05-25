use std::sync::Arc;
use viontin::Router;
use viontin::{Request, Response, StatusCode};

mod home;
mod blog;

pub fn register(router: Router) -> Router {
    router
        .get("/", Arc::new(home::index))
        .get("/features", Arc::new(home::features))
        .get("/about", Arc::new(home::about))
        .get("/contact", Arc::new(home::contact))
        .post("/contact", Arc::new(contact_submit))
        .get("/blog", Arc::new(home::blog))
        .get("/blog/:slug", Arc::new(blog::post))
        .get("/api/hello", Arc::new(api::hello))
        .get("/api/echo", Arc::new(api::echo))
        .post("/api/echo", Arc::new(api::echo_post))
        .get("/assets/tailwind.css", Arc::new(serve_css))
}

fn serve_css(_req: Request) -> Response {
    let path = std::path::Path::new("assets/tailwind.css");
    if path.exists() {
        match std::fs::read_to_string(path) {
            Ok(css) => Response::text(&css).with_header("content-type", "text/css; charset=utf-8"),
            Err(_) => Response::new(StatusCode::SERVER_ERROR).with_body("Failed to read CSS"),
        }
    } else {
        Response::new(StatusCode::NOT_FOUND)
            .with_body("Run the app to generate CSS")
    }
}

fn contact_submit(req: Request) -> Response {
    let body = req.body_str();
    let name = extract_form_field(body, "name").unwrap_or_else(|| "Anonymous".into());
    let email = extract_form_field(body, "email").unwrap_or_else(|| "unknown@example.com".into());
    let _message = extract_form_field(body, "message").unwrap_or_default();

    println!("  [contact] Message from {} <{}>", name, email);

    Response::html(&format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>Message Sent — Viontin</title>
        <link rel="stylesheet" href="/assets/tailwind.css">
        <style>:root{{--brand:#3b82f6;}}</style>
        </head><body class="bg-gray-50 text-gray-900" style="display:flex;align-items:center;justify-content:center;min-height:100vh">
        <div class="bg-white border border-gray-200 rounded p-8 shadow-sm text-center" style="max-width:480px">
        <div style="font-size:48px;margin-bottom:16px">&#9989;</div>
        <h1 class="text-2xl font-bold mb-4">Message Sent!</h1>
        <p class="text-gray-600 mb-6">Thanks, {}! We'll get back to you at <strong>{}</strong>.</p>
        <a href="/" class="text-white px-6 py-3 rounded font-semibold" style="background:var(--brand);display:inline-block">Back to Home</a>
        </div></body></html>"#,
        name, email
    ))
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

mod api {
    use viontin::{Request, Response};

    pub fn hello(_req: Request) -> Response {
        Response::json(&serde_json::json!({
            "message": "Hello from Viontin!",
            "version": "0.1.0",
            "status": "ok"
        }))
    }

    pub fn echo(req: Request) -> Response {
        Response::json(&serde_json::json!({
            "method": "GET",
            "query": req.uri.query,
            "path": req.uri.path,
            "headers": {
                "host": req.header("host").unwrap_or(""),
                "user-agent": req.header("user-agent").unwrap_or(""),
            }
        }))
    }

    pub fn echo_post(req: Request) -> Response {
        Response::json(&serde_json::json!({
            "method": "POST",
            "body": req.body_str(),
            "content-type": req.header("content-type").unwrap_or(""),
        }))
    }
}
