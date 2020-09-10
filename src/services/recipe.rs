use crate::{
    collection,
    models::{
        ingredient::Ingredient,
        recipe::{NewRecipe, Recipe},
    },
models::direction::Direction};
use bson::doc;
use juniper::FieldError;
use uuid::Uuid;

/**
 * User creates a recipe
 */
pub fn create_recipe(user_id: &str, new_recipe: NewRecipe) -> Result<Recipe, FieldError> {

    let mut id_ingredients = vec![];

    // todo, determine if id should be mapped or generated here instead of front end

    for ingredient in new_recipe.ingredients {
        id_ingredients.push(Ingredient {
            id: Uuid::new_v4().to_string(),
            amount: ingredient.amount,
            name: ingredient.name,
            unit: ingredient.unit,
        })
    }

    let mut id_directions = vec![];

    for direction in new_recipe.directions {
        id_directions.push(Direction {
            id: Uuid::new_v4().to_string(),
            imageUrl: direction.imageUrl,
            instruction: direction.instruction,
            step: direction.step,
        })
    }

    let recipe = Recipe {
        id: Uuid::new_v4().to_string(),
        creatorid: user_id.to_string(),
        name: new_recipe.name,
        description: new_recipe.description,
        ingredients: id_ingredients,
        category: new_recipe.category,
        prepTime: new_recipe.prepTime,
        cookTime: new_recipe.cookTime,
        servingSize: new_recipe.servingSize,
        creationDate: new_recipe.creationDate,
        directions: id_directions,
        imageUrl: new_recipe.imageUrl,
        reviewCount: 0,
        reviewDistribution: vec![],
        reviewRating: 0,
        reviews: vec![],
    };

    let mut coll = collection("recipe");
    let serialized_recipe = bson::to_bson(&recipe)?;

    if let bson::Bson::Document(document) = serialized_recipe {
        coll.insert_one(document, None)?;
        let recipe_document = coll
            .find_one(Some(doc! { "id" => recipe.to_owned().id }), None)?
            .expect("Document not found");

        let recipe_result = bson::from_bson(bson::Bson::Document(recipe_document))?;

        // Recipe has been created, so insert the id into the user
        coll = collection("user");
        let filter = doc! {"id" : user_id};
        let update = doc! {"$addToSet": {"recipes": recipe.to_owned().id}};
        coll.update_one(filter.clone(), update, None)
            .expect("Failed to add recipe to user");

        Ok(recipe_result)
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
