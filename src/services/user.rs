use crate::{
    database::Database,
    models::user::{NewUser, User}, collection,
};
use bson::doc;
use juniper::FieldError;

pub fn get_user(db: &Database, id: &str) -> Result<Vec<User>, FieldError> {
    let coll = collection("user");
    let filter = doc! {"id" : id};
    let cursor = coll.find(filter, None).unwrap();

    let mut results: Vec<User> = vec![];

    for result in cursor {
        match result {
            Ok(doc) => {
                let user: Option<User> = bson::from_bson(bson::Bson::Document(doc)).ok();
                println!("res: {:?}", user);
                results.push(user.unwrap());
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    Ok(results)
}
