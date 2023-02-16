use crate::entity::{prelude::*, todo};
use actix_web::web::Json;
use log::debug;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveValue::NotSet};
use sea_orm::{entity::*, query::*, DeriveEntityModel, DeleteResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoRequest {
  pub title: String,
  pub completed: bool,
}

#[derive(Clone, Debug)]
pub struct TodoRepository {
  pub db_conn: DatabaseConnection
}

impl TodoRepository {
  pub async fn get_todos(&self) -> Vec<todo::Model> {
    Todo::find()
      .all(&self.db_conn)
      .await
      .expect("Failed to get todos")
  }

  pub async fn get_todo_by_id(
    &self,
    id: i32
  ) -> Option<todo::Model> {
    Todo::find_by_id(id)
      .one(&self.db_conn)
      .await
      .expect("Failed to get todo")
  }

  pub async fn create_todo(
    &self,
    new_todo: Json<crate::repository::todos::TodoRequest>,
  ) -> Option<todo::Model> {
    let todo = todo::ActiveModel {
      id: NotSet,
      title: ActiveValue::Set(new_todo.title.to_owned()),
      completed: ActiveValue::Set(new_todo.completed.to_owned()),
    };

    let todo: todo::Model = todo.insert(&self.db_conn).await.unwrap();
    debug!("Created todo: {}", todo.title);
    return todo.into();
  }

  pub async fn update_todo(
    &self,
    id: i32,
    updated_todo: Json<crate::repository::todos::TodoRequest>,
  ) -> Option<todo::Model> {
    let todo = Todo::find_by_id(id)
      .one(&self.db_conn)
      .await
      .expect("Failed to get todo");

    match todo {
      Some(todo) => {
        let mut todo: todo::ActiveModel = todo.into();
        todo.title = ActiveValue::Set(updated_todo.title.to_owned());
        todo.completed = ActiveValue::Set(updated_todo.completed.to_owned());

        let todo: todo::Model = todo.update(&self.db_conn).await.unwrap();
        debug!("Updated todo: {}", todo.title);
        return todo.into();
      },
      None => {
        return None;
      }
    }
  }

  pub async fn delete_todo(
    &self,
    id: i32,
  ) -> DeleteResult {
    let res: DeleteResult = Todo::delete_by_id(id)
      .exec(&self.db_conn)
      .await
      .unwrap();

    debug!("Deleted todo with id: {}", id);
    return res.into();
  }
}