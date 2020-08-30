use serde::{Deserialize, Serialize};

// ListItem type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct ListItem {
    id: String,
    name: String,
    checked: bool,
    creationDate: String,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewListItem {
    name: String,
    creationDate: String,
}
