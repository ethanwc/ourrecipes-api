#![feature(proc_macro_hygiene, decl_macro)]

use jsonwebtokens_cognito::{Error, KeySet};
use juniper::RootNode;
use rocket::{response::content, routes, Rocket, State};
use tokio::runtime::Runtime;

mod database;


use database::Database;

 
#[macro_use]
extern crate rocket;
use serde::Deserialize;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
struct User {
    userName: String,
}

/**
 * Add new userid to application database
 */
#[post("/", data = "<user>")]
fn add_user(user: Json<User>) {
    println!("{}", user.userName);
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .manage(Database::init())
        .mount(
            "/",
            routes![add_user],
        )
}
