use serde::{Deserialize, Serialize};

// Direction type
#[derive(Serialize, Deserialize, Debug)]
pub struct Direction {
    id: String,
    instruction: String,
    step: String,
    imageUrl: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewDirection {
    instruction: String,
    step: String,
    imageUrl: String,
}

#[juniper::object(description = "A Direction")]
impl Direction {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn instruction(&self) -> &str {
        &self.instruction
    }
    pub fn step(&self) -> &str {
        &self.step
    }
    pub fn imageUrl(&self) -> &str {
        &self.imageUrl
    }
}
