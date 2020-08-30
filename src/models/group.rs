use serde::{Deserialize, Serialize};

// Group type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Group {
    id: String,
    name: String,
    creatorid: String,
    memberids: Vec<String>,
    creationDate: String,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewGroup {
    name: String,
    creatorid: String,
    memberids: Vec<String>,
    creationDate: String,
}
