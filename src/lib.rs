#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{Rocket, routes, get};
use tokio::runtime::Runtime;
use jsonwebtokens_cognito::KeySet;


mod model;
mod schema;

use juniper::{RootNode};
use model::Database;
use rocket::response::content;
use rocket::State;
use schema::{MutationRoot, Query};
use mongodb::{Client, Collection};
use lazy_static::lazy_static;

#[get("/auth")]
fn auth() -> &'static str {
    println!("test");
    Runtime::new()
    .expect("Failed to create Tokio runtime")
    .block_on(test3());
    println!("test2");
    "Auth checked"
}

async fn test3() -> Result<(), Box<dyn std::error::Error>> { 
    println!("test3");
    let keyset = KeySet::new("us-west-2", "us-west-2_JIyd7gfYd")?;
    println!("test4");

    let verifier = keyset.new_id_token_verifier(&["76pmarb1rb17n3tctrdmsqv60e"]).build()?;
    println!("test5");

    let result = keyset.verify("eyJraWQiOiJsTFNSbXdNZzBweGdEd01tdVI2clROZnVPaTFRbEtpdHA2aDhnUlFKd1drPSIsImFsZyI6IlJTMjU2In0.eyJhdF9oYXNoIjoiWkIzYXk3eTFOMmdwcGtjSXBkNlZNZyIsInN1YiI6IjQ4MjM0ZTkwLWJkM2ItNDEwZi1iOTBlLWMxNjE2YjZiNjljMCIsImNvZ25pdG86Z3JvdXBzIjpbInVzLXdlc3QtMl9KSXlkN2dmWWRfR29vZ2xlIl0sImVtYWlsX3ZlcmlmaWVkIjpmYWxzZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLXdlc3QtMi5hbWF6b25hd3MuY29tXC91cy13ZXN0LTJfSkl5ZDdnZllkIiwiY29nbml0bzp1c2VybmFtZSI6Imdvb2dsZV8xMDU5MDM3MjM1MTUxNDYxODAxODciLCJub25jZSI6IlhvUUdpbW1rQ3lyd2JOVmtmZHUzQXBpWmlEU0p2ejdpN000LXdVSmtWdlk4cEs2TkFhOEM5MmRCc0R4Rk9XelN5VC1SWjhlaTBOZmNMQURhX29HN3RiQXJWdlhFelZ4dEpad0JxYUV4WFpHUkhyMUVqRzFHc2dPSGJQSC11elVNOXB5UmZlU3daTmFSdDVmTGRWLUlraXlKbFFWc1ExbnZ5T1U3azV6WXRHUSIsImF1ZCI6Ijc2cG1hcmIxcmIxN24zdGN0cmRtc3F2NjBlIiwiaWRlbnRpdGllcyI6W3sidXNlcklkIjoiMTA1OTAzNzIzNTE1MTQ2MTgwMTg3IiwicHJvdmlkZXJOYW1lIjoiR29vZ2xlIiwicHJvdmlkZXJUeXBlIjoiR29vZ2xlIiwiaXNzdWVyIjpudWxsLCJwcmltYXJ5IjoidHJ1ZSIsImRhdGVDcmVhdGVkIjoiMTU5NDUyMDYwMjg4MCJ9XSwidG9rZW5fdXNlIjoiaWQiLCJhdXRoX3RpbWUiOjE1OTQ2ODk4MTQsIm5hbWUiOiJFdGhhbiBDaGVhdGhhbSIsImV4cCI6MTU5NDY5MzQxNCwiaWF0IjoxNTk0Njg5ODE0LCJlbWFpbCI6ImV0aGFud2NoZWF0aGFtQGdtYWlsLmNvbSJ9.mFMcqaTB2Ri92atg2WMhoPi9cNXETSBAn_B_9agBsrdl13Gmon1C8Rzo2cIv27sOC6Z2ut4_fzqKfufYOB33VJkNaO2aXFHJpccoh2kr-sIoKag31q0tNaN7hYOpEAF7sl7X-wvOY6EbGkBAHruf8rvFGFzXMxMHcwJpSdV7_5mQLx4pCQQnsqaeZWuDUpBa39nnxAx_7l3vYmsRG-G5hNboANjw6WQJ1_Sa219Yh5025PDtuvOO2mmu4ICPT1h6JiiVuq4T7qQQsad-cV6u-rtAqD_e4Xfx5jFiEJBdWxhMT0kVBNWK--Oaa2Xh9ozCK51bIfmQoC6ZiwFCy_cFLw", 
    &verifier).await;
    
    println!("{:?}", keyset);
    println!("{:?}", result);


    println!("ran");
    println!("test6");
    Ok(())
}

type Schema = RootNode<'static, Query, MutationRoot>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}


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


pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![auth, graphiql, get_graphql_handler, post_graphql_handler])
}
