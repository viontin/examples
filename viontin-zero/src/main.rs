use viontin::boot;

mod commands;
mod routes;

fn main() {
    boot()
        .command(commands::greet::Greet)
        .routes(routes::register)
        .serve("127.0.0.1:3000");
}
