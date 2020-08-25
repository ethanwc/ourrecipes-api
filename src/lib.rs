#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{ routes, Rocket};
use tokio::runtime::Runtime;
use rocket_contrib::json::Json;
use mongodb::{Client, Collection};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

 
#[macro_use]
extern crate rocket;


lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    Client::with_uri_str("mongodb+srv://username:password@cluster.mongodb.net")
    .expect("Failed to initialize standalone client.")
}

fn collection(coll_name: &str) -> Collection {
    MONGO.database("collection").collection(coll_name)
}




#[derive(Serialize, Deserialize)]
pub struct NewUser {
    userName: String,
}

/**
 * Add new userid to application database
 */
#[post("/", data = "<user>")]
fn add_user(user: Json<NewUser>) {
    println!("{}", user.userName);
}

use bson::doc;


async fn test3(new_user: NewUser) -> Result<(), Box<dyn std::error::Error>> { 
    let coll = collection("user");
    let serialized_member = bson::to_bson(&new_user)?;

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;
        //todo: use actual user id
        let member_document = coll
            .find_one(Some(doc! { "id" => "asdf" }), None)?
            .expect("Document not found");

        let human = bson::from_bson(bson::Bson::Document(member_document))?;
        Ok(human)
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}


#[post("/abc", data = "<user>")]
fn add_user2(user: Json<NewUser>) {
    let user2 = NewUser {
        userName: user.userName.to_owned()
    };
     Runtime::new()
    .expect("Failed to create Tokio runtime")
    .block_on(test3(user2)).expect("asdf");
}


pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![add_user],
        )
}
