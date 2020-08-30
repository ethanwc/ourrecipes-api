use super::ingredient::Ingredient;
use crate::services::ingredient::get_ingredient;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Ingredient3 {
//     pub(crate) id: String,
//     pub(crate) name: String,
//     pub(crate) amount: i32,
//     pub(crate) unit: String,
// }

// #[derive(Serialize, Deserialize, juniper::GraphQLObject)]
// pub struct Person {
//     name: String,
//     age: i32,
// }

// #[derive(Serialize, Deserialize, juniper::GraphQLObject)]
// struct Ingredient2 {
//     address: Option<String>,  // Converted into String (nullable)
//     inhabitants: Vec<Person>, // Converted into [Person!]!
// }

// User type
// #[derive(Serialize, Deserialize, juniper::GraphQLObject)]
#[derive(Serialize, Deserialize)]

pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub bio: String,
    // creationDate: String,
    // pub ingredients: Vec<Ingredient>,
    // groups: Vec<String>,
    pub bookmarks: Vec<String>,
    // shoppinglist: Vec<String>,
    followers: Vec<String>,
    following: Vec<String>,
    // reviews: Vec<String>,
    // recipes: Vec<String>,
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
        // &self.name
        "name_select_test"
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
    // pub fn creationDate(&self) -> &str {
    //     &self.creationDate
    // }
    // pub fn ingredients(&self) -> Vec<Ingredient> {
    //     // get_ingredient("157895bd-d323-48bb-8d20-7d251631a230").unwrap()
    //     // vec![
    //     //     Person {
    //     //         age: 123,
    //     //         name: "asdf".to_string(),
    //     //     },
    //     //     Person {
    //     //         age: 123,
    //     //         name: "asdf".to_string(),
    //     //     },
    //     // ]
    //     self.ingredients.to_owned()
    // }
    // pub fn recipes(&self) -> Vec<String> {
    //     self.recipes.to_owned()
    // }
    // pub fn groups(&self) -> Vec<String> {
    //     self.recipes.to_owned()
    // }
    pub fn bookmarks(&self) -> Vec<String> {
        self.bookmarks.to_owned()
    }
    // pub fn shoppinglist(&self) -> Vec<String> {
    //     self.shoppinglist.to_owned()
    // }
    pub fn followers(&self) -> Vec<String> {
        self.followers.to_owned()
    }
    pub fn following(&self) -> Vec<String> {
        self.following.to_owned()
    }
    // pub fn reviews(&self) -> Vec<String> {
    //     self.reviews.to_owned()
    // }
    pub fn pictures(&self) -> Vec<String> {
        self.pictures.to_owned()
    }
}
