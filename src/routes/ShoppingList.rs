use crate::db;
use crate::models::ShoppingList::ShoppingList;

#[rocket::get("/list")]
pub fn mylist() -> &'static str {
    "Hello, world! s asoj ajkl"
}
