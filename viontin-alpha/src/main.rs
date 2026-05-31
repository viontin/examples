#![allow(dead_code)]

use viontin::boot;
use viontin::gem::GemBuilder;
use viontin_gem_tailwind::Tailwind;

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
        .gem(Tailwind::load())
        .command(commands::greet::Greet)
        .command(commands::greet::Calc)
        .serve("127.0.0.1:3000");
}
