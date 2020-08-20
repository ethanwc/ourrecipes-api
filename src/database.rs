use mongodb::{Client, Collection};

fn create_mongo_client() -> Client {
    Client::with_uri_str(
        "mongodb+srv://ourrecipes-temp:Vpzk0A4RUxHRsWYB@cluster0.1bw4q.mongodb.net",
    )
    .expect("Failed to initialize standalone client.")
}

pub struct Database {
    db: Client,
}

impl Database {
    pub fn init() -> Database {
        Database {
            db: create_mongo_client(),
        }
    }
    pub fn collection(&self, coll_name: &str) -> Collection {
        self.db.database("collection").collection(&coll_name)
    }
}
