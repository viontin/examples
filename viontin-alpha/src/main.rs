use viontin::boot;

mod routes;
mod models;
mod commands;
mod events;
mod jobs;
mod mail;
mod notifications;
mod queries;

fn main() {
    boot()
        .gem(viontin_gem_tailwind::Gem)
        .routes(routes::register)
        .command(commands::greet::Greet)
        .command(commands::greet::Calc)
        .serve("127.0.0.1:3000");
}
