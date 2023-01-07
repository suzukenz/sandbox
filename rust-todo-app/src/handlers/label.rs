use super::ValidatedJson;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use std::sync::Arc;

use crate::repositories::label::{CreateLabel, LabelRepository};

#[derive(Clone)]
pub struct LabelState<T: LabelRepository> {
    pub repository: Arc<T>,
}

pub async fn create_label<T: LabelRepository>(
    State(label_state): State<LabelState<T>>,
    ValidatedJson(payload): ValidatedJson<CreateLabel>,
) -> Result<impl IntoResponse, StatusCode> {
    let label = label_state
        .repository
        .create(payload.name)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(label)))
}

pub async fn all_label<T: LabelRepository>(
    State(label_state): State<LabelState<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let labels = label_state.repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(labels)))
}

pub async fn delete_label<T: LabelRepository>(
    State(label_state): State<LabelState<T>>,
    Path(id): Path<i32>,
) -> StatusCode {
    label_state
        .repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
