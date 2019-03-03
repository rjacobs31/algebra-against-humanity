#![warn(clippy::all)]

extern crate actix_web;
use actix_web::{server, App, HttpRequest};

mod cards;
mod game;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello, world!"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();
}
