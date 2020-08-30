use super::{listitem::ListItem, recipe::Recipe, review::Review};
use crate::services::{recipe::get_recipe, review::get_review, user::get_user};
use serde::{Deserialize, Serialize};

// User type
#[derive(Clone, Serialize, Deserialize)]

pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub bio: String,
    pub creationDate: String,
    // groups: Vec<String>,
    pub bookmarks: Vec<String>,
    pub shoppinglist: Vec<ListItem>,
    pub followers: Vec<String>,
    pub following: Vec<String>,
    pub reviews: Vec<String>,
    pub recipes: Vec<String>,
    pub pictures: Vec<String>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject, Debug)]
pub struct NewUser {
    name: String,
    email: String,
    creationDate: String,
}

#[juniper::object(description = "A user")]
impl User {
    pub fn id(&self) -> &str {
        &self.id
    }
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
    pub fn recipes(&self) -> Vec<Recipe> {
        get_recipe(self.recipes.to_owned()).unwrap()
    }
    // pub fn groups(&self) -> Vec<String> {
    //     self.recipes.to_owned()
    // }
    pub fn bookmarks(&self) -> Vec<String> {
        self.bookmarks.to_owned()
    }
    pub fn shoppinglist(&self) -> Vec<ListItem> {
        self.shoppinglist.to_owned()
    }
    pub fn followers(&self) -> Vec<User> {
        get_user(self.followers.to_owned()).unwrap()
    }
    pub fn following(&self) -> Vec<User> {
        get_user(self.following.to_owned()).unwrap()
    }
    pub fn reviews(&self) -> Vec<Review> {
        get_review(self.reviews.to_owned()).unwrap()
    }
    pub fn pictures(&self) -> Vec<String> {
        self.pictures.to_owned()
    }
}
