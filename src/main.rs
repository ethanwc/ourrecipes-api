#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api", routes![routes::ShoppingList::mylist,])
}

fn main() {
    rocket().launch();
}
