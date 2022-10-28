use crate::{models::todo_model::Todo, db::mongodb_repo::{MongoRepo, Error}};
use actix_web::{
    get,
    post,
    Responder,
    http::StatusCode,
    web::{Data, Json, Path},
    error::ResponseError,
};

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ObjectIdError(_) => StatusCode::BAD_REQUEST,
            Self::MongoDBError(_) | Self::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TaskNotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[get("/todo/{id}")]
pub async fn get_todo(db: Data<MongoRepo>, path: Path<String>) -> Result<impl Responder, Error> {
    db.get_todo(&path).await.map(Json)
}

#[post("/todo")]
pub async fn create_todo(db: Data<MongoRepo>, todo: Json<Todo>) -> Result<impl Responder, Error> {
    db.create_todo(todo.into_inner()).await.map(Json)
}
