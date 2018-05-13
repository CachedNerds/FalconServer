#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Falcon Server"
}

#[get("/ready")]
fn ready() -> &'static str {
    "ready"
}

#[get("/health")]
fn health() -> &'static str {
    "healthy"
}



fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
