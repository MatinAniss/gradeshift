use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    AppState,
    http::{HttpError, middleware::Session},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_organisations_with_permission))
        .route("/{id}", get(get_organisation_by_id))
}

#[derive(Serialize, FromRow)]
struct Organisation {
    id: Uuid,
    name: String,
}

#[derive(Serialize)]
struct GetOrganisationsWithPermissionResponsePayload {
    organisations: Vec<Organisation>,
}

async fn get_organisations_with_permission(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
) -> Result<impl IntoResponse, HttpError> {
    let organisations: Vec<Organisation> = sqlx::query_as("SELECT o.id, o.name FROM organisations o JOIN organisation_permissions op ON o.id = op.organisation_id WHERE op.user_id = $1;")
        .bind(&session.user.id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| HttpError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to retrieve organisations".to_string(),
        })?;

    Ok(Json(GetOrganisationsWithPermissionResponsePayload {
        organisations,
    }))
}

#[derive(Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "organisation_role", rename_all = "UPPERCASE")]
enum OrganisationPermission {
    Admin,
    Staff,
    User,
}

#[derive(Serialize, FromRow)]
struct Subject {
    id: Uuid,
    name: String,
}

#[derive(Serialize)]
struct GetOrganisationByIdResponsePayload {
    id: Uuid,
    name: String,
    subjects: Vec<Subject>,
}

async fn get_organisation_by_id(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    let (organisation_permission,): (OrganisationPermission,) = sqlx::query_as(
        "SELECT permission FROM organisation_permissions WHERE user_id = $1 AND organisation_id = $2;",
    )
    .bind(&session.user.id)
    .bind(&id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| HttpError {
        status_code: StatusCode::UNAUTHORIZED,
        message: "Unauthorized".to_string(),
    })?;

    let organisation: Organisation =
        sqlx::query_as("SELECT id, name FROM organisations WHERE id = $1;")
            .bind(&id)
            .fetch_one(&state.db)
            .await
            .map_err(|_| HttpError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal Server Error".to_string(),
            })?;

    let subjects: Vec<Subject> = if matches!(
        organisation_permission,
        OrganisationPermission::Admin | OrganisationPermission::Staff
    ) {
        sqlx::query_as("SELECT id, name FROM subjects WHERE organisation_id = $1;")
            .bind(&organisation.id)
            .fetch_all(&state.db)
            .await
            .map_err(|_| HttpError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to retrieve subjects".to_string(),
            })?
    } else {
        sqlx::query_as("SELECT s.id, s.name FROM subjects s JOIN subject_assignmentssa ON s.id = sa.subject_id WHERE sa.user_id = $1 AND s.organisation_id = $2;")
        .bind(&session.user.id)
        .bind(&organisation.id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| HttpError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to retrieve subjects".to_string(),
        })?
    };

    Ok(Json(GetOrganisationByIdResponsePayload {
        id: organisation.id,
        name: organisation.name,
        subjects,
    }))
}
