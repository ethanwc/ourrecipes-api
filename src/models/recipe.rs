use super::ingredient::{Ingredient, NewIngredient};
use serde::{Deserialize, Serialize};

// Recipe type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Recipe {
    pub id: String,
    // creatorid: String,
    pub name: String,
    // imageUrl: String,
    // reviewCount: i32,
    // reviewRating: i32,
    // creationDate: String,
    // prepTime: String,
    // cookTime: String,
    // servingSize: i32,
    // category: String,
    pub ingredients: Vec<Ingredient>,
    // directions: Vec<String>,
    // reviews: Vec<String>,
    // reviewDistribution: Vec<i32>,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewRecipe {
    pub name: String,
    // creatorid: String,
    // creationDate: String,
    // imageUrl: String,
    // reviewCount: i32,
    // reviewRating: i32,
    // prepTime: String,
    // cookTime: String,
    // servingSize: i32,
    // category: String,
    // a: Ingredient,
    pub ingredients: Vec<NewIngredient>,
    // directions: Vec<String>,
}
