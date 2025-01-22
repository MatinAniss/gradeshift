use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    http::{middleware::Session, HttpError},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/{id}", get(get_subject_by_id))
}

#[derive(Serialize, FromRow)]
struct Task {
    id: Uuid,
    name: String,
}

#[derive(Serialize, FromRow)]
struct Subject {
    id: Uuid,
    name: String,
}

#[derive(Serialize)]
struct GetSubjectByIdResponsePayload {
    id: Uuid,
    name: String,
    tasks: Vec<Task>,
}

async fn get_subject_by_id(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    sqlx::query("SELECT 1 FROM subject_assignments WHERE user_id = $1 AND subject_id = $2;")
        .bind(&session.user.id)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| HttpError {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Unauthorized".to_string(),
        })?;

    let subject: Subject = sqlx::query_as("SELECT id, name FROM subjects WHERE id = $1;")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| HttpError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error".to_string(),
        })?;

    let tasks: Vec<Task> = sqlx::query_as("SELECT t.id, t.name FROM tasks t JOIN task_assignments ta ON t.id = ta.task_id WHERE ta.user_id = $1 AND t.subject_id = $2;")
        .bind(&session.user.id)
        .bind(&id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| HttpError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error".to_string(),
        })?;

    Ok(Json(GetSubjectByIdResponsePayload {
        id: subject.id,
        name: subject.name,
        tasks,
    }))
}
