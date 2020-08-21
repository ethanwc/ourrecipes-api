use crate::{
    database::Database,
    models::{
        human::{Human, NewHuman},
        user::{NewUser, User},
    },
    services::{
        human::{create_human, get_human},
        user::{create_user, get_user},
    },
};

use juniper::FieldResult;

// The root query object of the schema
pub struct Query;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
impl Query {
    #[graphql(arguments(id(description = "id of the human")))]
    fn human(database: &Database, id: String) -> FieldResult<Vec<Human>> {
        get_human(database, &id)
    }
    #[graphql(arguments(id(description = "id of the user")))]
    fn user(database: &Database, id: String) -> FieldResult<Vec<User>> {
        get_user(database, &id)
    }
}

// Mutation object query for api
pub struct MutationRoot;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
impl MutationRoot {
    fn create_human(database: &Database, new_human: NewHuman) -> FieldResult<Human> {
        create_human(database, new_human)
    }
    fn create_human(database: &Database, new_user: NewUser) -> FieldResult<User> {
        create_user(database, new_user)
    }
}
