use crate::{collection, models::user::User};
use bson::doc;
use juniper::FieldError;


/*
*
*    // if let bson::Bson::Document(document) = serialized_recipe {
   //     recipe_collection.insert_one(document, None)?;
   //     let recipe_document = recipe_collection
   //         .find_one(Some(doc! { "id" => "asdf" }), None)?
   //         .expect("Document not found");

   //     let recipe = bson::from_bson(bson::Bson::Document(recipe_document))?;
   // }
*/
