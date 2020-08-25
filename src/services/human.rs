use crate::{
    auth,
    database::Database,
    models::{user::{User, NewUser}, human::{Human, NewHuman}},
};
use bson::doc;
use juniper::{graphql_value, FieldError};


pub fn create_user(db: &Database, new_user: NewUser) -> Result<User, FieldError> {
    let coll = db.collection("user");
    let serialized_member = bson::to_bson(&new_user)?;

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;
        //todo: use actual user id
        let member_document = coll
            .find_one(Some(doc! { "id" => "asdf" }), None)?
            .expect("Document not found");

        let human = bson::from_bson(bson::Bson::Document(member_document))?;
        Ok(human)
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}

pub fn create_human(db: &Database, new_human: NewHuman) -> Result<Human, FieldError> {
    let coll = db.collection("human");
    let serialized_member = bson::to_bson(&new_human)?;

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;
        let member_document = coll
            .find_one(Some(doc! { "id" => new_human.id }), None)?
            .expect("Document not found");

        let human = bson::from_bson(bson::Bson::Document(member_document))?;
        Ok(human)
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }
}

// verify token via auth in create routes only

pub fn get_human(db: &Database, id: &str) -> Result<Vec<Human>, FieldError> {
    // call auth function in here i guess
    let coll = db.collection("human");
    let filter = doc! {"id" : id};
    let cursor = coll.find(filter, None).unwrap();

    let mut results: Vec<Human> = vec![];

    // let authR = auth();

    // if !authR.auth {
    //     return Err(FieldError::new(
    //         "Access requested was declined: 401",
    //         graphql_value!({ "internal_error": "Connection refused" }),
    //     ));
    // }

    // println!("{}:{}", authR.auth, authR.user_id);

    for result in cursor {
        match result {
            Ok(doc) => {
                let human: Option<Human> = bson::from_bson(bson::Bson::Document(doc)).ok();
                println!("res: {:?}", human);
                results.push(human.unwrap());
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    Ok(results)
}
