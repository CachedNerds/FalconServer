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

#[get("/list")]
fn list() -> &'static str {
    ""
}

#[post("/join/<id>")]
fn join(id: u32) ->  String {
    format!("{}", id)
}

#[post("/leave/<id>")]
fn leave(id: u32) ->  String {
    format!("{}", id)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
