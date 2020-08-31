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
        creatorid: user_id.to_string(),
        name: new_recipe.name,
        ingredients: id_ingredients,
    };

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
}

/**
 * Returns recipe(s)
 */
pub fn get_recipe(ids: Vec<String>) -> Result<Vec<Recipe>, FieldError> {
    let coll = collection("recipe");
    let filter = doc! {"id": {"$in": ids}};

    let cursor = coll.find(filter, None).unwrap();

    let mut results: Vec<Recipe> = vec![];

    for result in cursor {
        match result {
            Ok(doc) => {
                let recipe: Option<Recipe> = bson::from_bson(bson::Bson::Document(doc)).ok();
                results.push(recipe.unwrap());
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    Ok(results)
}
