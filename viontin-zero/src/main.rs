use viontin::boot;

mod commands;
mod routes;

fn main() {
    boot()
        .command(commands::greet::Greet)
        .serve("127.0.0.1:3000");
}
