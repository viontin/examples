use viontin::{Request, Response, StatusCode};

pub fn post(req: Request) -> Response {
    let path = &req.uri.path;
    let slug = path.strip_prefix("/blog/").unwrap_or("").to_string();

    if slug.is_empty() || slug.contains('/') {
        return Response::not_found().with_body("Post not found".to_string());
    }

    let template = include_str!("../../html/blog-post.html");
    let content = match slug.as_str() {
        "hello-viontin" => template
            .replace("__TITLE__", "Hello, Viontin!")
            .replace("__DATE__", "May 18, 2026")
            .replace("__AUTHOR__", "The Viontin Team")
            .replace("__CONTENT__", "\
<p>We are excited to announce the first release of Viontin — Zero to One, Scale-up Easily.</p>\
<p>Viontin provides everything you need to build modern applications: an HTTP server, ORM, CLI toolkit, caching, mail, notifications, events, queues, scheduling, and much more — all in one binary.</p>\
<p>What sets Viontin apart is its 6-level architecture. From a single-file prototype (Level 0) to a multi-region fleet (Level 5), the framework grows with your project. Each level adds guardrails without removing capabilities.</p>\
<p>Built entirely in Rust, Viontin offers memory safety, zero-cost abstractions, and native performance — without requiring a garbage collector or runtime.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Get Started</h2>\
<p>Install Viontin and create your first project:</p>\
<pre class='bg-gray-100 p-4 rounded font-mono text-sm mt-2'>cargo install viontin
viontin new my-app
cd my-app
viontin dev</pre>"),

        "why-rust" => template
            .replace("__TITLE__", "Why Rust for Web Development")
            .replace("__DATE__", "May 15, 2026")
            .replace("__AUTHOR__", "The Viontin Team")
            .replace("__CONTENT__", "\
<p>Rust has been the most loved programming language on Stack Overflow for years running. But why does it matter for web development?</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Memory Safety</h2>\
<p>Rust's ownership model eliminates entire categories of bugs at compile time — no null pointer dereferences, no dangling pointers, no buffer overflows. This means fewer security vulnerabilities in production.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Zero-Cost Abstractions</h2>\
<p>High-level abstractions in Rust compile down to the same machine code as hand-written low-level code. You get expressiveness without sacrificing performance.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>No Garbage Collector</h2>\
<p>Unlike Go, Java, or C#, Rust does not require a garbage collector. Memory is managed at compile time through ownership rules. This means predictable performance, lower memory usage, and faster startup times.</p>"),

        "levels-explained" => template
            .replace("__TITLE__", "The 6 Levels Explained")
            .replace("__DATE__", "May 10, 2026")
            .replace("__AUTHOR__", "The Viontin Team")
            .replace("__CONTENT__", "\
<p>Viontin defines 6 levels of architectural maturity. Each level adds structure, automation, and safety — without breaking the previous level.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Level 0 — Zero</h2>\
<p>Single file, zero configuration, no boilerplate. Perfect for prototyping and learning.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Level 1 — Indie</h2>\
<p>Modular structure with src/routes/, src/models/, and src/commands/. Auto-generated API docs.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Level 2 — Team</h2>\
<p>Compiler-enforced domain boundaries. #[domain] annotations and viontin check --arch for architecture validation.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Level 3 — Enterprise</h2>\
<p>Microservices, event-driven communication, and shared contracts between services.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Level 4 — Fleet</h2>\
<p>Kubernetes orchestration, OpenTelemetry tracing, and platform tooling.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Level 5 — Mesh</h2>\
<p>Multi-region deployment, service mesh integration, and chaos engineering.</p>"),

        "getting-started" => template
            .replace("__TITLE__", "Getting Started with Viontin")
            .replace("__DATE__", "May 5, 2026")
            .replace("__AUTHOR__", "The Viontin Team")
            .replace("__CONTENT__", "\
<p>This guide walks you through creating your first Viontin project, step by step.</p>\
<h2 class='text-xl font-bold mt-6 mb-3'>Installation</h2>\
<p>Install the Viontin CLI:</p>\
<pre class='bg-gray-100 p-4 rounded font-mono text-sm mt-2'>cargo install viontin</pre>\
<h2 class='text-xl font-bold mt-6 mb-3'>Create a Project</h2>\
<pre class='bg-gray-100 p-4 rounded font-mono text-sm mt-2'>viontin new my-app
cd my-app</pre>\
<h2 class='text-xl font-bold mt-6 mb-3'>Run the Server</h2>\
<pre class='bg-gray-100 p-4 rounded font-mono text-sm mt-2'>viontin dev</pre>\
<p>Your application will be available at http://127.0.0.1:3000.</p>"),

        _ => {
            let msg = format!("Post '{}' not found. <a href='/blog'>Back to blog</a>", slug);
            return Response::new(StatusCode::NOT_FOUND).with_body(msg);
        }
    };

    Response::html(&content)
}
