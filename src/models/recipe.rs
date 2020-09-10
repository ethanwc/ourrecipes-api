use super::{ingredient::{Ingredient, NewIngredient}, direction::Direction, direction::NewDirection};
use serde::{Deserialize, Serialize};

// Recipe type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Recipe {
    pub id: String,
    pub creatorid: String,
    pub name: String,
    pub description: String,
    pub imageUrl: String,
    pub reviewCount: i32,
    pub reviewRating: i32,
    pub creationDate: String,
    pub prepTime: String,
    pub cookTime: String,
    pub servingSize: String,
    pub category: String,
    pub ingredients: Vec<Ingredient>,
    pub directions: Vec<Direction>,
    pub reviews: Vec<String>,
    pub reviewDistribution: Vec<i32>,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewRecipe {
    pub name: String,
    pub description: String,
    pub creationDate: String,
    pub imageUrl: String,
    pub prepTime: String,
    pub cookTime: String,
    pub servingSize: String,
    pub category: String,
    pub ingredients: Vec<NewIngredient>,
    pub directions: Vec<NewDirection>,
}