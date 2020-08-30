use super::ingredient::{NewIngredient, Ingredient};
use serde::{Deserialize, Serialize};

// Recipe type
#[derive(Serialize, Deserialize, juniper::GraphQLObject)]
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
    pub(crate) ingredients: Vec<Ingredient>,
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

// #[juniper::object(description = "A recipe")]
// impl Recipe {
//     pub fn id(&self) -> &str {
//         &self.id
//     }
//     // pub fn creatorid(&self) -> &str {
//     //     &self.creatorid
//     // }
//     // pub fn name(&self) -> &str {
//     //     &self.name
//     // }
//     // pub fn imageUrl(&self) -> &str {
//     //     &self.imageUrl
//     // }
//     // pub fn reviewCount(&self) -> i32 {
//     //     self.reviewCount
//     // }
//     // pub fn reviewRating(&self) -> i32 {
//     //     self.reviewRating
//     // }
//     // pub fn creationDate(&self) -> &str {
//     //     &self.creationDate
//     // }
//     // pub fn prepTime(&self) -> &str {
//     //     &self.prepTime
//     // }
//     // pub fn cookTime(&self) -> &str {
//     //     &self.cookTime
//     // }
//     // pub fn servingSize(&self) -> i32 {
//     //     self.servingSize
//     // }
//     // pub fn category(&self) -> &str {
//     //     &self.category
//     // }
//     // pub fn cookTime(&self) -> &str {
//     //     &self.cookTime
//     // }
//     // pub fn ingredients(&self) -> Vec<String> {
//     //     self.ingredients.to_owned()
//     // }
//     // pub fn directions(&self) -> Vec<String> {
//     //     self.directions.to_owned()
//     // }
//     // pub fn bookmarks(&self) -> Vec<String> {
//     //     self.bookmarks.to_owned()
//     // }
//     // pub fn reviews(&self) -> Vec<String> {
//     //     self.reviews.to_owned()
//     // }
//     // pub fn reviewDistribution(&self) -> Vec<i32> {
//     //     self.reviewDistribution.to_owned()
//     // }
// }
