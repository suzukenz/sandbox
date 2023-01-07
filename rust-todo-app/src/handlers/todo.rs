use super::ValidatedJson;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use std::sync::Arc;

use crate::repositories::todo::{CreateTodo, TodoRepository, UpdateTodo};

#[derive(Clone)]
pub struct TodoState<T: TodoRepository> {
    pub repository: Arc<T>,
}

pub async fn create_todo<T: TodoRepository>(
    State(todo_state): State<TodoState<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = todo_state
        .repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<T: TodoRepository>(
    State(todo_state): State<TodoState<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = todo_state
        .repository
        .find(id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    State(todo_state): State<TodoState<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todos = todo_state.repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(todos)))
}

pub async fn update_todo<T: TodoRepository>(
    State(todo_state): State<TodoState<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = todo_state
        .repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    State(todo_state): State<TodoState<T>>,
    Path(id): Path<i32>,
) -> StatusCode {
    todo_state
        .repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
