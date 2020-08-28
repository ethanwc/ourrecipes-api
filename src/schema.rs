use crate::{
    database::Database,
    models::{
        human::Human,
        ingredient::{Ingredient, NewIngredient}, user::User,
    },
    services::{
        human::get_human,
        ingredient::{create_ingredient, get_ingredient}, user::get_user,
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
    #[graphql(arguments(id(description = "id of the user")))]
    fn user(database: &Database, id: String) -> FieldResult<Vec<User>> {
        get_user(database, &id)
    }
    #[graphql(arguments(id(description = "id of the ingredient")))]
    fn ingredient(database: &Database, id: String) -> FieldResult<Vec<Ingredient>> {
        get_ingredient(database, &id)
    }
}

// Mutation object query for api
pub struct MutationRoot;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
impl MutationRoot {
    fn create_ingredient(
        database: &Database,
        new_ingredient: NewIngredient,
    ) -> FieldResult<Ingredient> {
        create_ingredient(database, new_ingredient)
    }
}
