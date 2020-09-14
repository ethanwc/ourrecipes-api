use crate::{collection, models::user::User, models::recipe::Recipe};
use bson::doc;
use juniper::FieldError;

/**
 * Returns user(s)
 */
pub fn get_user(ids: Vec<String>) -> Result<Vec<User>, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id": {"$in": ids}};

    let cursor = coll.find(filter, None).unwrap();

    let mut results: Vec<User> = vec![];

    for result in cursor {
        match result {
            Ok(doc) => {
                let user: Option<User> = bson::from_bson(bson::Bson::Document(doc)).ok();
                // println!("res: {:?}", user);
                results.push(user.unwrap());
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    Ok(results)
}

/**
 * User updates profile picture
 */
pub fn update_picture(user_id: &str, photo_uri: &str) -> Result<User, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id" : user_id};
    let update = doc! {"$set": {"photo": photo_uri}};
    coll.update_one(filter.clone(), update, None)
        .expect("Failed to create bookmark");

    // Get user after update
    let user_document = coll
        .find_one(filter.clone(), None)
        .expect("Failed to update profile photo");
    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;
    Ok(user)
}

/**
 * User bookmarks a recipe
 */
pub fn create_bookmark(user_id: &str, bookmark_id: &str) -> Result<Recipe, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id" : user_id};
    let update = doc! {"$addToSet": {"bookmarks": bookmark_id}};
    coll.update_one(filter.clone(), update, None)
        .expect("Failed to create bookmark");

    // Get recipe after bookmark to display
    let recipe_coll = collection("recipe");
    let recipe_filter = doc! {"id" : bookmark_id};

    let recipe_document = recipe_coll
    .find_one(recipe_filter, None)
    .expect("Failed to find recipe");
    let recipe = bson::from_bson(bson::Bson::Document(recipe_document.unwrap()))?;

    Ok(recipe)
}

/**
* User unbookmarks a recipe
*/
pub fn delete_bookmark(user_id: &str, bookmark_id: &str) -> Result<User, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id" : user_id};
    let update = doc! {"$pull": {"bookmarks": bookmark_id}};
    coll.update_one(filter.clone(), update, None)
        .expect("Failed to remove bookmark");

    // Get user after update
    let user_document = coll
        .find_one(filter.clone(), None)
        .expect("Failed to remove bookmark");
    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;
    Ok(user)
}

/**
 * User adds a photo
 */
pub fn create_photo(user_id: &str, photo_uri: &str) -> Result<User, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id" : user_id};
    let update = doc! {"$addToSet": {"pictures": photo_uri}};
    coll.update_one(filter.clone(), update, None)
        .expect("Failed to create photo");

    // Get user after update
    let user_document = coll
        .find_one(filter.clone(), None)
        .expect("Failed to create photo");
    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;
    Ok(user)
}

/**
 * User removes a photo
 */
pub fn delete_photo(user_id: &str, photo_uri: &str) -> Result<User, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id" : user_id};
    let update = doc! {"$pull": {"pictures": photo_uri}};
    coll.update_one(filter.clone(), update, None)
        .expect("Failed to delete photo");

    // Get user after update
    let user_document = coll
        .find_one(filter.clone(), None)
        .expect("Failed to delete photo");
    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;
    Ok(user)
}

/**
 * Update user bio
 */
pub fn update_bio(user_id: &str, bio: &str) -> Result<User, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id" : user_id};
    let update = doc! {"$set": {"bio": bio}};
    coll.update_one(filter.clone(), update, None)
        .expect("Failed to update bio");

    // Get user after update
    let user_document = coll
        .find_one(filter.clone(), None)
        .expect("Failed to update bio");
    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;
    Ok(user)
}

/**
 * Follow a user
 */
pub fn follow(user_id: &str, follow_id: &str) -> Result<User, FieldError> {
    let coll = collection("user");
    let user_filter = doc! {"id" : user_id};
    let follow_filter = doc! {"id" : follow_id};
    // User follows someone
    let user_update = doc! {"$addToSet": {"following": "testfollow"}};
    let follow_update = doc! {"$addToSet": {"followers": "testfollow"}};

    // Update followed user followers
    coll.find_one_and_update(follow_filter, follow_update, None)
        .expect("Failed to follow target");

    // Update user following list
    coll.update_one(user_filter.clone(), user_update, None)
        .expect("Failed to follow via user");

    // Return updated user
    let user_document = coll
        .find_one(user_filter.clone(), None)
        .expect("Failed to get user");

    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;

    Ok(user)
}

/**
 * Unfollow a user
 */
pub fn unfollow(user_id: &str, unfollow_id: &str) -> Result<User, FieldError> {
    let coll = collection("user");
    let user_filter = doc! {"id" : user_id};
    let follow_filter = doc! {"id" : unfollow_id};
    // User follows someone
    let user_update = doc! {"$pull": {"following": "testfollow"}};
    let follow_update = doc! {"$pull": {"followers": "testfollow"}};

    // Update followed user followers
    coll.find_one_and_update(follow_filter, follow_update, None)
        .expect("Failed to unfollow target");

    // Update user following list
    coll.update_one(user_filter.clone(), user_update, None)
        .expect("Failed to unfollow via user");

    // Return updated user
    let user_document = coll
        .find_one(user_filter.clone(), None)
        .expect("Failed to get user");

    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;

    Ok(user)
}
