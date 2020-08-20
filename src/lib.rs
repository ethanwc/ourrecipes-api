#![feature(proc_macro_hygiene, decl_macro)]

use jsonwebtokens_cognito::KeySet;
use juniper::RootNode;
use rocket::{get, response::content, routes, Rocket, State};
use tokio::runtime::Runtime;

mod database;
mod models;
mod schema;
mod services;

use database::Database;
use schema::{MutationRoot, Query};

#[get("/auth")]
fn auth() -> &'static str {
    println!("test");
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(check_auth())
        .expect("Authorization Check Failed!");
    "Auth checked"
}

async fn check_auth() -> Result<(), Box<dyn std::error::Error>> {
    let keyset = KeySet::new("us-west-2", "us-west-2_JIyd7gfYd")?;

    let verifier = keyset
        .new_id_token_verifier(&["76pmarb1rb17n3tctrdmsqv60e"])
        .build()?;

    let result = keyset.verify("eyJraWQiOiJsTFNSbXdNZzBweGdEd01tdVI2clROZnVPaTFRbEtpdHA2aDhnUlFKd1drPSIsImFsZyI6IlJTMjU2In0.eyJhdF9oYXNoIjoiWkIzYXk3eTFOMmdwcGtjSXBkNlZNZyIsInN1YiI6IjQ4MjM0ZTkwLWJkM2ItNDEwZi1iOTBlLWMxNjE2YjZiNjljMCIsImNvZ25pdG86Z3JvdXBzIjpbInVzLXdlc3QtMl9KSXlkN2dmWWRfR29vZ2xlIl0sImVtYWlsX3ZlcmlmaWVkIjpmYWxzZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLXdlc3QtMi5hbWF6b25hd3MuY29tXC91cy13ZXN0LTJfSkl5ZDdnZllkIiwiY29nbml0bzp1c2VybmFtZSI6Imdvb2dsZV8xMDU5MDM3MjM1MTUxNDYxODAxODciLCJub25jZSI6IlhvUUdpbW1rQ3lyd2JOVmtmZHUzQXBpWmlEU0p2ejdpN000LXdVSmtWdlk4cEs2TkFhOEM5MmRCc0R4Rk9XelN5VC1SWjhlaTBOZmNMQURhX29HN3RiQXJWdlhFelZ4dEpad0JxYUV4WFpHUkhyMUVqRzFHc2dPSGJQSC11elVNOXB5UmZlU3daTmFSdDVmTGRWLUlraXlKbFFWc1ExbnZ5T1U3azV6WXRHUSIsImF1ZCI6Ijc2cG1hcmIxcmIxN24zdGN0cmRtc3F2NjBlIiwiaWRlbnRpdGllcyI6W3sidXNlcklkIjoiMTA1OTAzNzIzNTE1MTQ2MTgwMTg3IiwicHJvdmlkZXJOYW1lIjoiR29vZ2xlIiwicHJvdmlkZXJUeXBlIjoiR29vZ2xlIiwiaXNzdWVyIjpudWxsLCJwcmltYXJ5IjoidHJ1ZSIsImRhdGVDcmVhdGVkIjoiMTU5NDUyMDYwMjg4MCJ9XSwidG9rZW5fdXNlIjoiaWQiLCJhdXRoX3RpbWUiOjE1OTQ2ODk4MTQsIm5hbWUiOiJFdGhhbiBDaGVhdGhhbSIsImV4cCI6MTU5NDY5MzQxNCwiaWF0IjoxNTk0Njg5ODE0LCJlbWFpbCI6ImV0aGFud2NoZWF0aGFtQGdtYWlsLmNvbSJ9.mFMcqaTB2Ri92atg2WMhoPi9cNXETSBAn_B_9agBsrdl13Gmon1C8Rzo2cIv27sOC6Z2ut4_fzqKfufYOB33VJkNaO2aXFHJpccoh2kr-sIoKag31q0tNaN7hYOpEAF7sl7X-wvOY6EbGkBAHruf8rvFGFzXMxMHcwJpSdV7_5mQLx4pCQQnsqaeZWuDUpBa39nnxAx_7l3vYmsRG-G5hNboANjw6WQJ1_Sa219Yh5025PDtuvOO2mmu4ICPT1h6JiiVuq4T7qQQsad-cV6u-rtAqD_e4Xfx5jFiEJBdWxhMT0kVBNWK--Oaa2Xh9ozCK51bIfmQoC6ZiwFCy_cFLw", 
    &verifier).await;

    println!("{:?}", keyset);
    println!("{:?}", result);
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

pub fn rocket() -> Rocket {
    rocket::ignite()
        .manage(Database::init())
        .manage(Schema::new(Query, MutationRoot))
        .mount(
            "/",
            routes![auth, graphiql, get_graphql_handler, post_graphql_handler],
        )
}
