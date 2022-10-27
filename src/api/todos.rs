use crate::{models::todo_model::Todo, db::mongodb_repo::{MongoRepo, Error}};
use actix_web::{
    get,
    post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/todo/{id}")]
pub async fn get_todo(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    db.get_todo(&path)
        .await
        .map(|todo| HttpResponse::Ok().json(todo))
        .unwrap_or_else(|err| match err {
            Error::ObjectIdError(err) => HttpResponse::BadRequest().body(err.to_string()),
            _ => HttpResponse::InternalServerError().body(err.to_string())
        })
}

#[post("/todo")]
pub async fn create_todo(db: Data<MongoRepo>, new_todo: Json<Todo>) -> HttpResponse {
    let data = Todo::new(&new_todo.task, new_todo.completed);
    db.create_todo(data)
        .await
        .map(|todo| HttpResponse::Ok().json(todo))
        .unwrap_or_else(|err| HttpResponse::InternalServerError().body(err.to_string()))
}
