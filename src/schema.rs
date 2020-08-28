use crate::{
    database::Database,
    models::{
        ingredient::{Ingredient, NewIngredient},
        user::User,
    },
    services::{
        ingredient::{create_ingredient, get_ingredient},
        user::get_user,
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
    fn user(id: String) -> FieldResult<Vec<User>> {
        get_user(&id)
    }
    #[graphql(arguments(id(description = "id of the ingredient")))]
    fn ingredient(id: String) -> FieldResult<Vec<Ingredient>> {
        get_ingredient(&id)
    }
}

// Mutation object query for api
pub struct MutationRoot;

#[juniper::object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
)]
impl MutationRoot {
    fn create_ingredient(new_ingredient: NewIngredient) -> FieldResult<Ingredient> {
        create_ingredient(new_ingredient)
    }
}
