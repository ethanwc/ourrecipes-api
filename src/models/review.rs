use serde::{Deserialize, Serialize};

// Review type
#[derive(Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Review {
    pub id: String,
    pub creatorid: String,
    pub recipeid: String,
    pub review: String,
    pub rating: i32,
}

#[derive(Clone, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewReview {
    pub creatorid: String,
    pub recipeid: String,
    pub review: String,
    pub rating: i32,
}
