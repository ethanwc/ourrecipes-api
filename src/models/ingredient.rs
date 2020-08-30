use serde::{Deserialize, Serialize};

// Ingredient type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub amount: i32,
    pub unit: String,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewIngredient {
    pub name: String,
    pub amount: i32,
    pub unit: String,
}
