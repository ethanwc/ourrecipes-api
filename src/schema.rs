use crate::{
    database::Database,
    models::{recipe, human::{Human, NewHuman}},
    services::human::{create_human, get_human},
};

use juniper::FieldResult;
use recipe::NewRecipe;

// The root query object of the schema
pub struct Query;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
impl Query {
    #[graphql(arguments(id(description = "id of the human")))]
    fn human(database: &Database, id: String, recipe: NewRecipe) -> FieldResult<Vec<Human>> {
        get_human(database, &id)
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
}
