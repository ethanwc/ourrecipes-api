use crate::{
    context::Context,
    models::{
        recipe::{NewRecipe, Recipe},
        review::Review,
        user::User,
    },
    services::{
        recipe::{create_recipe, get_recipe},
        review::get_review,
        user::{
            create_bookmark, create_photo, delete_bookmark, delete_photo, follow, update_bio,
            update_picture,
        },
        user::{get_user, unfollow},
    }};

use juniper::{FieldResult, graphql_value};
use juniper::FieldError;

// The root query object of the schema
pub struct Query;

#[juniper::object(
    Context = Context,
    Scalar = juniper::DefaultScalarValue,
)]
impl Query {
    #[graphql(arguments(id(description = "ids of the users")))]
    fn user(ids: Vec<String>) -> FieldResult<Vec<User>> {
        get_user(ids)
    }
    #[graphql(arguments(id(description = "ids of the recipes")))]
    fn recipe(ids: Vec<String>) -> FieldResult<Vec<Recipe>> {
        get_recipe(ids)
    }
    #[graphql(arguments(id(description = "ids of the reviews")))]
    fn review(ids: Vec<String>) -> FieldResult<Vec<Review>> {
        get_review(ids)
    }
}

// Mutation object query for api
pub struct MutationRoot;

#[juniper::object(
    Context = Context,
    Scalar = juniper::DefaultScalarValue,
)]
impl MutationRoot {
    fn create_bookmark(context: &Context, jwt: String, recipe_id: String) -> FieldResult<User> {
        let authCheck = context.to_owned().authorize(jwt);

        if authCheck.auth {
            create_bookmark(&authCheck.user_id, &recipe_id)
        } else {
            Err(FieldError::new(
                "Auth declined",
                graphql_value!({ "access_declined": "Connection refused" })
            ))
        }
    }
    fn delete_bookmark(user_id: String, recipe_id: String) -> FieldResult<User> {
        delete_bookmark(&user_id, &recipe_id)
    }
    fn create_photo(user_id: String, photo_uri: String) -> FieldResult<User> {
        create_photo(&user_id, &photo_uri)
    }
    fn delete_photo(user_id: String, photo_uri: String) -> FieldResult<User> {
        delete_photo(&user_id, &photo_uri)
    }
    fn update_picture(context: &Context, jwt: String, photo_uri: String) -> FieldResult<User> {
        let authCheck = context.to_owned().authorize(jwt);

        if authCheck.auth {
            update_picture(&authCheck.user_id, &photo_uri)
        } else {
            Err(FieldError::new(
                "Auth declined",
                graphql_value!({ "access_declined": "Connection refused" })
            ))
        }
    }
    fn update_bio(user_id: String, bio: String) -> FieldResult<User> {
        update_bio(&user_id, &bio)
    }
    fn follow_user(user_id: String, follow_id: String) -> FieldResult<User> {
        follow(&user_id, &follow_id)
    }
    fn unfollow_user(user_id: String, follow_id: String) -> FieldResult<User> {
        unfollow(&user_id, &follow_id)
    }
    fn create_recipe(user_id: String, recipe: NewRecipe) -> FieldResult<Recipe> {
        create_recipe(&user_id, recipe)
    }
}
