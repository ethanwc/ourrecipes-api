#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/hello", routes![world]);
}