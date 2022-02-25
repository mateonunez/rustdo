use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::response::Response;

pub type Todos = Response<Todo>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
  pub id: String,
  pub title: String,
  pub description: String,
  pub completed: bool,
  pub created_at: DateTime<Utc>,
}

impl Todo {
  pub fn new(title: String, description: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      title,
      description,
      completed: false,
      created_at: Utc::now(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoRequest {
  pub title: Option<String>,
  pub description: Option<String>,
}

impl TodoRequest {
  pub fn to_todo(&self) -> Option<Todo> {
    match &self.title {
      Some(title) => match &self.description {
        Some(description) => Some(Todo::new(title.to_string(), description.to_string())),
        None => None,
      },
      None => None,
    }
  }
}

// list 50 last todos `/todos`
#[get("/todos")]
pub async fn list() -> HttpResponse {
  let todos = Todos { results: vec![] };

  HttpResponse::Ok()
    .content_type("application/json")
    .json(todos)
}