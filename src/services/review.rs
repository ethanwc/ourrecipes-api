use crate::{
    collection,
    models::{user::User, review::{NewReview, Review}},
};
use bson::doc;
use juniper::FieldError;
use uuid::Uuid;

/**
 * User creates a review
 */
pub fn create_review(user_id: &str, new_review: NewReview) -> Result<Vec<Review>, FieldError> {
    let review = Review {
        id: Uuid::new_v4().to_string(),
        creatorid: user_id.to_string(),
        recipeid: new_review.recipeid.to_string(),
        review: new_review.review,
        rating: new_review.rating,
    };

    let review_coll = collection("review");
    let serialized_review = bson::to_bson(&review)?;

    if let bson::Bson::Document(document) = serialized_review {
        review_coll.insert_one(document, None)?;
        let review_document = review_coll
            .find_one(Some(doc! { "id" => review.id.clone() }), None)?
            .expect("Document not found");

        // Add id of review to user's reviews after creation
        let user_coll = collection("user");
        let user_filter = doc! {"id" : user_id};
        let user_update = doc! {"$addToSet": {"reviews": review.id.clone()}};
        user_coll
            .update_one(user_filter, user_update, None)
            .expect("Failed to update profile picture");

        let review = bson::from_bson(bson::Bson::Document(review_document))?;
        Ok(review)
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}

/**
 * Returns review(s)
 */
pub fn get_review(ids: Vec<String>) -> Result<Vec<Review>, FieldError> {
    let coll = collection("review");
    let filter = doc! {"id": {"$in": ids}};

    let cursor = coll.find(filter, None).unwrap();

    let mut results: Vec<Review> = vec![];

    for result in cursor {
        match result {
            Ok(doc) => {
                let review: Option<Review> = bson::from_bson(bson::Bson::Document(doc)).ok();
                results.push(review.unwrap());
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    Ok(results)
}

/**
* User deletes a review
*/
pub fn delete_review(user_id: &str, review_id: &str) -> Result<User, FieldError> {
    
    // Delete review from reviews collection
    let review_coll = collection("review");
    let review_filter = doc! {"id" : user_id};
    review_coll.delete_one(review_filter, None).expect("Failed to delete review");
    
    // Delete review id from user's reviews
    let user_coll = collection("user");
    let user_filter = doc! {"id" : user_id};
    let update = doc! {"$pull": {"bookmarks": review_id}};
    user_coll.update_one(user_filter.clone(), update, None)
        .expect("Failed to remove bookmark");

    // Get user after update
    let user_document = user_coll
        .find_one(user_filter.clone(), None)
        .expect("Failed to remove bookmark");
    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;
    Ok(user)
}