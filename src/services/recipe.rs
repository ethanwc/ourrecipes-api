use crate::{
    collection,
    models::{
        ingredient::Ingredient,
        recipe::{NewRecipe, Recipe},
        user::User,
    },
};
use bson::doc;
use juniper::FieldError;

/**
 * User creates a recipe
 */
pub fn create_recipe(new_recipe: NewRecipe, user_id: String) -> Result<User, FieldError> {
    let user_collection = collection("user");
    let recipe_collection = collection("recipe");
    let serialized_recipe = bson::to_bson(&new_recipe)?;

    let recipe = Recipe {
        id: "asdf".to_string(),
        name: new_recipe.name,
        // ingredients: vec![Ingredient {
        //     amount: 1,
        //     id: "Asdf".to_string(),
        //     name: "asdf".to_string(),
        //     unit: "sasdf".to_string(),
        // }],
    };

    if let bson::Bson::Document(document) = serialized_recipe {
        recipe_collection.insert_one(document, None)?;
        let recipe_document = recipe_collection
            .find_one(Some(doc! { "id" => "todo: uuidhere" }), None)?
            .expect("Document not found");

        let recipe = bson::from_bson(bson::Bson::Document(recipe_document))?;
    }

    let filter = doc! {"id" : user_id};
    let update = doc! {"$addToSet": {"recipes": "todo: new recipeid1234asdf"}};

    let user_document = user_collection
        .find_one_and_update(filter, update, None)
        .expect("Failed to create recipe");

    let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;

    Ok(user)
}
