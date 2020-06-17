#![feature(proc_macro_hygiene, decl_macro)]
// #![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, Rust!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}