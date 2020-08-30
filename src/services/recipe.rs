use crate::{
    collection,
    models::{
        ingredient::Ingredient,
        recipe::{NewRecipe, Recipe},
    },
};
use bson::doc;
use juniper::FieldError;
use uuid::Uuid;

/**
 * User creates a recipe
 */
pub fn create_recipe(user_id: &str, new_recipe: NewRecipe) -> Result<Recipe, FieldError> {
    // let recipe_collection = collection("recipe");

    let mut id_ingredients = vec![];

    for ingredient in new_recipe.ingredients {
        id_ingredients.push(Ingredient {
            amount: ingredient.amount,
            id: Uuid::new_v4().to_string(),
            name: ingredient.name,
            unit: ingredient.unit,
        })
    }

    let recipe = Recipe {
        id: Uuid::new_v4().to_string(),
        name: new_recipe.name,
        ingredients: id_ingredients,
    };

    // let serialized_member = bson::to_bson(&recipe)?;
    let coll = collection("recipe");
    let serialized_recipe = bson::to_bson(&recipe)?;

    if let bson::Bson::Document(document) = serialized_recipe {
        coll.insert_one(document, None)?;
        //todo: use actual user id
        let recipe_document = coll
            .find_one(Some(doc! { "id" => recipe.id }), None)?
            .expect("Document not found");

        let recipe = bson::from_bson(bson::Bson::Document(recipe_document))?;
        Ok(recipe)
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }

    // if let bson::Bson::Document(document) = serialized_recipe {
    //     recipe_collection.insert_one(document, None)?;
    //     let recipe_document = recipe_collection
    //         .find_one(Some(doc! { "id" => "asdf" }), None)?
    //         .expect("Document not found");

    //     let recipe = bson::from_bson(bson::Bson::Document(recipe_document))?;
    // }
    // let user_collection = collection("user");
    // let filter = doc! {"id" : user_id};
    // let update = doc! {"$addToSet": {"recipes": "todo: new recipeid1234asdf"}};

    // let user_document = user_collection
    //     .find_one_and_update(filter, update, None)
    //     .expect("Failed to create recipe");

    //     let user = bson::from_bson(bson::Bson::Document(user_document.unwrap()))?;

    // Ok(Recipe {
    //     id: "a".to_string(),
    //     name: "a".to_string()
    // })
}
