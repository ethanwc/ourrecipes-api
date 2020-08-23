#![feature(proc_macro_hygiene, decl_macro)]

use jsonwebtokens_cognito::{Error, KeySet};
use juniper::RootNode;
use rocket::{response::{content}, routes, Rocket, State};
use tokio::runtime::Runtime;

mod database;
mod models;
mod schema;
mod services;

use database::Database;
use schema::{MutationRoot, Query};

fn auth() -> bool {
    let v = Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(check_auth())
        .expect("Authorization Check Failed!");
        true
}

async fn check_auth() -> Result<bool, Error> {
    let keyset = KeySet::new("us-west-2", "us-west-2_JIyd7gfYd")?;

    let verifier = keyset
        .new_id_token_verifier(&["76pmarb1rb17n3tctrdmsqv60e"])
        .build()?;

    Ok(keyset.verify("eyJraWQiOiJsTFNSbXdNZzBweGdEd01tdVI2clROZnVPaTFRbEtpdHA2aDhnUlFKd1drPSIsImFsZyI6IlJTMjU2In0.eyJhdF9oYXNoIjoiWGo4WWZYNHpuNU0zYy12QXRvbkx3USIsInN1YiI6IjQ4MjM0ZTkwLWJkM2ItNDEwZi1iOTBlLWMxNjE2YjZiNjljMCIsImNvZ25pdG86Z3JvdXBzIjpbInVzLXdlc3QtMl9KSXlkN2dmWWRfR29vZ2xlIl0sImVtYWlsX3ZlcmlmaWVkIjpmYWxzZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLXdlc3QtMi5hbWF6b25hd3MuY29tXC91cy13ZXN0LTJfSkl5ZDdnZllkIiwiY29nbml0bzp1c2VybmFtZSI6Imdvb2dsZV8xMDU5MDM3MjM1MTUxNDYxODAxODciLCJub25jZSI6Ii1aWV9sM1RVUlRkVkl0VWxNWDFYbGVzZzFUcG9FZVVpNWl5OXlFVWctY3pqbXlFODJSS2NaT0V2UWRhUEJCQlM4dGFxSUhWTjJnbFlzX1VVMmEtbHRPRF9KNHpvdGx1LUVIYTVjSmg2Q1dBNkUyY0JmY2ktdEw2TXRDU0JOampzMUNwTjhEYXlYUHYzWjlRSlpsSWw2a2pEYVFBRXRWcWI5bGpZRVRDaGJjYyIsImF1ZCI6Ijc2cG1hcmIxcmIxN24zdGN0cmRtc3F2NjBlIiwiaWRlbnRpdGllcyI6W3sidXNlcklkIjoiMTA1OTAzNzIzNTE1MTQ2MTgwMTg3IiwicHJvdmlkZXJOYW1lIjoiR29vZ2xlIiwicHJvdmlkZXJUeXBlIjoiR29vZ2xlIiwiaXNzdWVyIjpudWxsLCJwcmltYXJ5IjoidHJ1ZSIsImRhdGVDcmVhdGVkIjoiMTU5NDUyMDYwMjg4MCJ9XSwidG9rZW5fdXNlIjoiaWQiLCJhdXRoX3RpbWUiOjE1OTgxNzAyNTMsIm5hbWUiOiJFdGhhbiBDaGVhdGhhbSIsImV4cCI6MTU5ODE3Mzg1MywiaWF0IjoxNTk4MTcwMjUzLCJlbWFpbCI6ImV0aGFud2NoZWF0aGFtQGdtYWlsLmNvbSJ9.Yj2eCLwG5EHxgVYKtF85lpA_RxH6tr5gA8rQ52aR3cvZQmwDVdCsfNF6weWnRzuOxK4HvJdNSTpspQgqmbMx8TqD18tL--JknZjtEyb_CRH3Jbn3fCLooTk9b_Igz6UOlHaUybWWRtzxC2_7httH6CAqzF4lCkeTVVCKEJJ2yoF8v8Jk2Zfn828--nqIFWgN6cEaEtYhXcdonD8vshUcccaQmUD6fKJVZec57Q93V4cM3q1inMlPXXXWIRqb6T4LHXdkqVtsaTZClNTXusyAwglhkfRmF9gmyFxwdGDVO7So93jcaA_VrBbp3vSnNEormroWd7FXHucv_-UvwOhZgg", 
    &verifier).await.is_ok())


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
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
