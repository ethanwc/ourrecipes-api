use crate::{
    database::Database,
    models::{
        ingredient::{Ingredient, NewIngredient},
        user::User,
    },
    services::{
        ingredient::{create_ingredient, get_ingredient},
        user::{create_bookmark, create_photo, delete_photo, follow, update_bio, update_picture},
        user::{get_user, unfollow},
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
    fn create_bookmark(user_id: String, recipe_id: String) -> FieldResult<User> {
        create_bookmark(&user_id, &recipe_id)
    }
    fn create_photo(user_id: String, photo_uri: String) -> FieldResult<User> {
        create_photo(&user_id, &photo_uri)
    }
    fn delete_photo(user_id: String, photo_uri: String) -> FieldResult<User> {
        delete_photo(&user_id, &photo_uri)
    }
    fn update_picture(user_id: String, photo_uri: String) -> FieldResult<User> {
        update_picture(&user_id, &photo_uri)
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
}
