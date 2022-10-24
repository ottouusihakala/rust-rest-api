use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, //modify here
    results::InsertOneResult,
    Client, Collection, options::ClientOptions,
};
use crate::models::todo_model::Todo;

pub struct MongoRepo {
    todos: Collection<Todo>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client_options = ClientOptions::parse(uri).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        // let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("todo");
        let todos = db.collection::<Todo>("todos");
        MongoRepo { todos }
    }

    pub async fn create_todo(&self, new_todo: Todo) -> Result<InsertOneResult, Error> {
        let new_doc = Todo {
            id: None,
            task: new_todo.task,
            completed: new_todo.completed
        };
        let todo = self
            .todos
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(todo)
    }

    pub async fn get_todo(&self, id: &String) -> Result<Todo, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let todo_doc = self
            .todos
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(todo_doc.unwrap())
    }
}