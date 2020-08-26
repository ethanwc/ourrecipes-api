#![feature(proc_macro_hygiene, decl_macro)]

use bson::doc;
use futures::executor::block_on;
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use rocket::{routes, Rocket};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    Client::with_uri_str("mongodb+srv://wtf:DtnGOXLlEy2GxLwO@cluster0.1bw4q.mongodb.net")
        .expect("Failed to initialize standalone client.")
}

fn collection(coll_name: &str) -> Collection {
    MONGO.database("ourrecipes").collection(coll_name)
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    userName: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
}

// Add new userid to application database
#[post("/", data = "<user>")]
fn new(user: Json<NewUser>) {
    let new_user = User {
        id: user.userName.to_owned(),
    };
    let future = create_user(new_user);
    block_on(future).expect("Failed to create new user.");
}

async fn create_user(new_user: User) -> Result<(), Box<dyn std::error::Error>> {
    let coll = collection("user");
    let serialized_member = bson::to_bson(&new_user)?;

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;
        coll.find_one(Some(doc! { "id" => new_user.id }), None)?
            .expect("Document not found");
        Ok(())
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![new])
}
