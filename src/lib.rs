#![feature(proc_macro_hygiene, decl_macro)]

use jsonwebtokens_cognito::{Error, KeySet};
use juniper::RootNode;
use rocket::{response::content, routes, Rocket, State};
use tokio::runtime::Runtime;

mod context;
mod models;
mod schema;
mod services;

use context::Context;
use lazy_static::lazy_static;
use mongodb::{Client, Collection};
use schema::{MutationRoot, Query};

type Schema = RootNode<'static, Query, MutationRoot>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/test")]
fn test() -> String {
    "worked".to_string()
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

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

pub fn rocket() -> Rocket {
    rocket::ignite()
        .manage(Context::new())
        .manage(Schema::new(Query, MutationRoot))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler, test],
        )
}
