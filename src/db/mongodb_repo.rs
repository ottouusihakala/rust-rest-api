use mongodb::{
    bson::{oid::ObjectId, doc},
    results::InsertOneResult,
    Client, Collection, options::ClientOptions,
};
use crate::models::todo_model::Todo;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid ObjectId")]
    ObjectIdError(#[from] mongodb::bson::oid::Error),
    #[error("MongoDB Error")]
    MongoDBError(#[from] mongodb::error::Error)
}

pub struct MongoRepo {
    todos: Collection<Todo>,
}

impl MongoRepo {
    pub async fn init(uri: &str) -> Result<Self, Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("todo");
        let todos = db.collection::<Todo>("todos");
        Ok(MongoRepo { todos })
    }

    pub async fn create_todo(&self, new_todo: Todo) -> Result<InsertOneResult, Error> {
        let new_doc = Todo::new(&new_todo.task, new_todo.completed);
        let todo = self
            .todos
            .insert_one(new_doc, None)
            .await?;
        Ok(todo)
    }

    pub async fn get_todo(&self, id: &str) -> Result<Option<Todo>, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let todo = self
            .todos
            .find_one(filter, None)
            .await?;
        Ok(todo)
    }
}
