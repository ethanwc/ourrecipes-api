use crate::services::{user::get_user, review::get_review};

use super::{ingredient::{Ingredient, NewIngredient}, direction::Direction, direction::NewDirection, user::User, review::Review};
use serde::{Deserialize, Serialize};

// Recipe type
#[derive(Clone, Serialize, Deserialize)]
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
/**
    pub reviews: Vec<String>,
    pub reviewDistribution: Vec<i32>,
 */

#[juniper::object(description = "A recipe")]
impl Recipe {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn creator(&self) -> User {
        get_user(vec![self.creatorid.to_owned()]).unwrap()[0].clone()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn imageUrl(&self) -> &str {
        &self.imageUrl
    }
    pub fn reviewCount(&self) -> &i32 {
        &self.reviewCount
    }
    pub fn reviewRating(&self) -> &i32 {
        &self.reviewRating
    }
    pub fn creationDate(&self) -> &str {
        &self.creationDate
    }
    pub fn prepTime(&self) -> &str {
        &self.prepTime
    }
    pub fn cookTime(&self) -> &str {
        &self.cookTime
    }
    pub fn servingSize(&self) -> &str {
        &self.servingSize
    }
    pub fn category(&self) -> &str {
        &self.category
    }
    pub fn ingredients(&self) -> Vec<Ingredient> {
        self.ingredients.to_owned()
    }
    pub fn directions(&self) -> Vec<Direction> {
        self.directions.to_owned()
    }
    pub fn reviews(&self) -> Vec<Review> {
        get_review(self.reviews.to_owned()).unwrap()
    }
    pub fn reviewDistribution(&self) -> Vec<i32> {
        self.reviewDistribution.to_owned()
    }
}
