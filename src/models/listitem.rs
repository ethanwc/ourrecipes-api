use serde::{Deserialize, Serialize};

// ListItem type
#[derive(Serialize, Deserialize, Debug)]
pub struct ListItem {
    id: String,
    name: String,
    checked: bool,
    creationDate: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewListItem {
    name: String,
    creationDate: String,
}

#[juniper::object(description = "A ListItem")]
impl ListItem {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn checked(&self) -> &bool {
        &self.checked
    }
    pub fn creationDate(&self) -> &str {
        &self.creationDate
    }
}
