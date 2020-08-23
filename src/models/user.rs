use serde::{Deserialize, Serialize};

// User type
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    email: String,
    photo: String,
    bio: String,
    creationDate: String,
    recipes: Vec<String>,
    groups: Vec<String>,
    bookmarks: Vec<String>,
    shoppinglist: Vec<String>,
    followers: Vec<String>,
    following: Vec<String>,
    reviews: Vec<String>,
    pictures: Vec<String>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewUser {
    name: String,
    email: String,
    creationDate: String,
}

#[juniper::object(description = "A user")]
impl User {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn photo(&self) -> &str {
        &self.photo
    }
    pub fn bio(&self) -> &str {
        &self.bio
    }
    pub fn creationDate(&self) -> &str {
        &self.creationDate
    }
    pub fn recipes(&self) -> Vec<String> {
        self.recipes.to_owned()
    }
    pub fn groups(&self) -> Vec<String> {
        self.recipes.to_owned()
    }
    pub fn bookmarks(&self) -> Vec<String> {
        self.bookmarks.to_owned()
    }
    pub fn shoppinglist(&self) -> Vec<String> {
        self.shoppinglist.to_owned()
    }
    pub fn followers(&self) -> Vec<String> {
        self.followers.to_owned()
    }
    pub fn following(&self) -> Vec<String> {
        self.following.to_owned()
    }
    pub fn reviews(&self) -> Vec<String> {
        self.reviews.to_owned()
    }
    pub fn pictures(&self) -> Vec<String> {
        self.pictures.to_owned()
    }
}