use serde::{Deserialize, Serialize};

// Direction type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Direction {
    id: String,
    instruction: String,
    step: String,
    imageUrl: String,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewDirection {
    instruction: String,
    step: String,
    imageUrl: String,
}
