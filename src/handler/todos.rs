use crate::entity::todo::Model as Todo;
use crate::AppState;
use actix_web::{get, post, web, web::Json, Error as ActixError, Responder, Result as ActixResult, Scope, put, delete};
use log::{debug};
use sea_orm::DeleteResult;
use serde_json::json;

#[get("")]
async fn get_todos(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
  let todos = state.todo_repository.get_todos().await;
  Ok(web::Json(todos))
}

#[get("/{id}")]
async fn get_todo_by_id(state: web::Data<AppState>, id: web::Path<i32>) -> ActixResult<impl Responder, ActixError> {
  let todo = state.todo_repository.get_todo_by_id(id.into_inner()).await;
  Ok(web::Json(todo))
}

#[post("")]
async fn create_todo(state: web::Data<AppState>, new_todo: Json<crate::repository::todos::TodoRequest>) -> ActixResult<impl Responder, ActixError> {
  let todo = state.todo_repository.create_todo(new_todo).await;
  Ok(web::Json(todo))
}

#[put("/{id}")]
async fn update_todo(state: web::Data<AppState>, id: web::Path<i32>, updated_todo: Json<crate::repository::todos::TodoRequest>) -> ActixResult<impl Responder, ActixError> {
  let todo = state.todo_repository.update_todo(id.into_inner(), updated_todo).await;
  Ok(web::Json(todo))
}

#[delete("/{id}")]
async fn delete_todo(state: web::Data<AppState>, id: web::Path<i32>) -> ActixResult<impl Responder, ActixError> {
  let todo: DeleteResult = state.todo_repository.delete_todo(id.into_inner()).await;
  Ok(web::Json(json!({ "message": "Todo(s) deleted successfully", "deleted": todo.rows_affected})))
}

pub fn todos_handler() -> Scope {
  web::scope("/todos")
    .service(get_todos)
    .service(get_todo_by_id)
    .service(create_todo)
    .service(update_todo)
    .service(delete_todo)
}