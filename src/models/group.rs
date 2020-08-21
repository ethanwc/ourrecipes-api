use serde::{Deserialize, Serialize};

// Group type
#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    id: String,
    name: String,
    creatorid: String,
    memberids: Vec<String>,
    creationDate: String,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewGroup {
    name: String,
    creatorid: String,
    memberids: Vec<String>,
    creationDate: String,
}

#[juniper::object(description = "A Group")]
impl Group {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn creatorid(&self) -> &str {
        &self.creatorid
    }
    pub fn memberids(&self) -> Vec<String> {
        self.memberids.to_owned()
    }
    pub fn creationDate(&self) -> &str {
        &self.creationDate
    }
}
