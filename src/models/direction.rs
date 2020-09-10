use serde::{Deserialize, Serialize};

// Direction type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Direction {
    pub id: String,
    pub instruction: String,
    pub step: String,
    pub imageUrl: String,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewDirection {
    pub id: String,
    pub instruction: String,
    pub step: String,
    pub imageUrl: String,
}
