use serde::{Deserialize, Serialize};

// Ingredient type
#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) amount: i32,
    pub(crate) unit: String,
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
