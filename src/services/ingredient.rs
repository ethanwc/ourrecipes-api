use crate::{
    collection,
    models::ingredient::{Ingredient, NewIngredient},
};
use bson::doc;
use juniper::FieldError;

pub fn create_ingredient(new_ingredient: NewIngredient) -> Result<Ingredient, FieldError> {
    let id_ingredient = Ingredient {
        id: "asdf".to_string(),
        amount: new_ingredient.amount,
        name: new_ingredient.name,
        unit: new_ingredient.unit,
    };

    let coll = collection("ingredient");
    let serialized_member = bson::to_bson(&id_ingredient)?;

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;
        //todo: use actual user id
        let ingredient_document = coll
            .find_one(Some(doc! { "id" => id_ingredient.id }), None)?
            .expect("Document not found");

        let ingredient = bson::from_bson(bson::Bson::Document(ingredient_document))?;
        Ok(ingredient)
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}

pub fn get_ingredient(id: &str) -> Result<Vec<Ingredient>, FieldError> {
    let coll = collection("ingredient");
    let filter = doc! {"id" : id};
    let cursor = coll.find(filter, None).unwrap();

    let mut results: Vec<Ingredient> = vec![];

    for result in cursor {
        match result {
            Ok(doc) => {
                let ingredient: Option<Ingredient> =
                    bson::from_bson(bson::Bson::Document(doc)).ok();
                // println!("res: {:?}", ingredient);
                results.push(ingredient.unwrap());
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    Ok(results)
}
