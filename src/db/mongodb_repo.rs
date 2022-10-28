use mongodb::{
    bson::{oid::ObjectId, doc},
    Client, Collection, options::ClientOptions,
};
use crate::models::todo_model::Todo;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid ObjectId")]
    ObjectIdError(#[from] mongodb::bson::oid::Error),
    #[error("MongoDB Error")]
    MongoDBError(#[from] mongodb::error::Error),
    #[error("Task not found")]
    TaskNotFound,
    #[error("Unexpected error")]
    UnexpectedError
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

    pub async fn create_todo(&self, mut todo: Todo) -> Result<ObjectId, Error> {
        todo.id = None;
        self
            .todos
            .insert_one(todo, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or(Error::UnexpectedError)
    }

    pub async fn get_todo(&self, id: &str) -> Result<Todo, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        self
            .todos
            .find_one(filter, None)
            .await?
            .ok_or(Error::TaskNotFound)
    }
}
