use crate::{models::todo_model::Todo, db::mongodb_repo::MongoRepo};
use actix_web::{
    get,
    post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/todo/{id}")]
pub async fn get_todo(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }
    let get_todo_result = db.get_todo(&id).await;
    match get_todo_result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[post("/todo")]
pub async fn create_todo(db: Data<MongoRepo>, new_todo: Json<Todo>) -> HttpResponse {
    let data = Todo {
        id: None,
        task: new_todo.task.to_owned(),
        completed: new_todo.completed.to_owned()
    };
    let create_todo_result = db.create_todo(data).await;
    match create_todo_result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}