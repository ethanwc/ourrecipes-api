use serde::{Deserialize, Serialize};

// Ingredient type

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Ingredient {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) amount: i32,
    pub(crate) unit: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewIngredient {
    pub name: String,
    pub amount: i32,
    pub unit: String,
}

// #[juniper::object(description = "A ingredient")]
// impl Ingredient {
//     pub fn id(&self) -> &str {
//         &self.id
//     }
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//     pub fn amount(&self) -> i32 {
//         self.amount
//     }
//     pub fn unit(&self) -> &str {
//         &self.unit
//     }
// }
