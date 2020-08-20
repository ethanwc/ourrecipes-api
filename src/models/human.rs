use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Human {
    pub(crate) id: String,
    pub(crate) name: String,
}

#[juniper::object(description = "A human")]
impl Human {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewHuman {
    pub(crate) id: String,
    pub(crate) name: String,
}
