use serde::{Deserialize, Serialize};

// Ingredient type
#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    id: String,
    name: String,
    amount: i32,
    unit: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewIngredient {
    name: String,
    amount: i32,
    unit: String,
}

#[juniper::object(description = "A ingredient")]
impl Ingredient {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn amount(&self) -> i32 {
        self.amount
    }
    pub fn unit(&self) -> &str {
        &self.unit
    }
}
