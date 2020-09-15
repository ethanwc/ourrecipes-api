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
    }, services::recipe::search_recipes, services::recipe::get_all_recipes};

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
    #[graphql(arguments(id(description = "gets all the recipes")))]
    fn getAllRecipes() -> FieldResult<Vec<Recipe>> {
        get_all_recipes()
    }
    #[graphql(arguments(id(description = "search term for the recipes")))]
    fn searchRecipe(search: String) -> FieldResult<Vec<Recipe>> {
        search_recipes(search)
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
    fn create_bookmark(context: &Context, jwt: String, recipe_id: String) -> FieldResult<Recipe> {
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
    fn delete_bookmark(context: &Context, jwt: String, recipe_id: String) -> FieldResult<User> {
        let authCheck = context.to_owned().authorize(jwt);

        if authCheck.auth {
            delete_bookmark(&authCheck.user_id, &recipe_id)
        } else {
            Err(FieldError::new(
                "Auth declined",
                graphql_value!({ "access_declined": "Connection refused" })
            ))
        }
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
    fn follow_user(context: &Context, jwt: String, follow_id: String) -> FieldResult<User> {
        let authCheck = context.to_owned().authorize(jwt);
        if authCheck.auth {
            follow(&authCheck.user_id, &follow_id)
        } else {
            Err(FieldError::new(
                "Auth declined",
                graphql_value!({ "access_declined": "Connection refused" })
            ))
        }
    }
    fn unfollow_user(context: &Context, jwt: String, follow_id: String) -> FieldResult<User> {
        let authCheck = context.to_owned().authorize(jwt);
        if authCheck.auth {
            unfollow(&authCheck.user_id, &follow_id)
        } else {
            Err(FieldError::new(
                "Auth declined",
                graphql_value!({ "access_declined": "Connection refused" })
            ))
        }
    }
    fn create_recipe(context: &Context, jwt: String, recipe: NewRecipe) -> FieldResult<Recipe> {
        let authCheck = context.to_owned().authorize(jwt);
        if authCheck.auth {
            create_recipe(&authCheck.user_id, recipe)
        } else {
            Err(FieldError::new(
                "Auth declined",
                graphql_value!({ "access_declined": "Connection refused" })
            ))
        }
    }
}
