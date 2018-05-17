#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

pub mod health;
pub mod index;
pub mod join;
pub mod leave;
pub mod list;
pub mod persist;
pub mod ready;
pub mod startup;

use startup::startup;

fn main() {
    let _ = startup();

    rocket::ignite()
        .mount(
            "/",
            routes![
                index::index,
                ready::ready,
                health::health,
                list::list,
                join::join,
                leave::leave
            ],
        )
        .launch();
}
