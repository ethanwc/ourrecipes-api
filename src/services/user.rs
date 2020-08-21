use crate::{
    database::Database,
    models::user::{NewUser, User},
};
use bson::doc;
use juniper::FieldError;
use uuid::Uuid;

pub fn create_user(db: &Database, new_user: NewUser) -> Result<User, FieldError> {
    let coll = db.collection("user");
    let serialized_member = bson::to_bson(&new_user)?;

    let my_uuid = Uuid::new_v4().to_string();
    println!("{}", my_uuid);

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;
        let member_document = coll
            .find_one(Some(doc! { "id" => my_uuid }), None)?
            .expect("Document not found");

        let user = bson::from_bson(bson::Bson::Document(member_document))?;
        Ok(user)
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}

pub fn get_user(db: &Database, id: &str) -> Result<Vec<User>, FieldError> {
    let coll = db.collection("user");
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
