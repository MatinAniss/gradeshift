use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    http::middleware::{auth_middleware, Session},
    AppState,
};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route(
            "/logout",
            get(logout).layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/refresh",
            get(refresh).layer(middleware::from_fn_with_state(state, auth_middleware)),
        )
}

struct AuthError {
    status_code: StatusCode,
    message: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct AuthErrorResponse<'a> {
            status_code: u16,
            message: &'a str,
        }

        let response = AuthErrorResponse {
            status_code: self.status_code.as_u16(),
            message: &self.message,
        };

        (self.status_code, Json(response)).into_response()
    }
}

#[derive(Deserialize)]
struct LoginRequestPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponsePayload {
    token: Token,
    user: User,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Token {
    token: String,
    created_at: chrono::DateTime<chrono::Utc>,
    expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequestPayload>,
) -> Result<impl IntoResponse, AuthError> {
    let (user_id, email, first_name, last_name, password_hash): (
        Uuid,
        String,
        String,
        String,
        String,
    ) = sqlx::query_as(
        "SELECT id, email, first_name, last_name, password_hash FROM users WHERE email = $1;",
    )
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await
    .map_err(|_| AuthError {
        status_code: StatusCode::UNAUTHORIZED,
        message: "Unauthorized".to_string(),
    })?;

    let password_hash = PasswordHash::new(&password_hash).map_err(|_| AuthError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Internal Server Error".to_string(),
    })?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &password_hash)
        .map_err(|_| AuthError {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Unauthorized".to_string(),
        })?;

    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let expires_at = chrono::Utc::now() + chrono::Duration::days(30);

    let (created_at,): (chrono::DateTime<chrono::Utc>,) = sqlx::query_as("INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING created_at;")
        .bind(&user_id)
        .bind(&token)
        .bind(&expires_at)
        .fetch_one(&state.db)
        .await
        .map_err(|_| AuthError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to issue session".to_string(),
        })?;

    Ok(Json(LoginResponsePayload {
        token: Token {
            token,
            created_at,
            expires_at,
        },
        user: User {
            id: user_id,
            email,
            first_name,
            last_name,
        },
    }))
}

#[derive(Serialize)]
struct LogoutResponsePayload {
    success: bool,
}

async fn logout(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
) -> Result<impl IntoResponse, AuthError> {
    sqlx::query("UPDATE sessions SET is_active = false WHERE id = $1")
        .bind(&session.token.id)
        .execute(&state.db)
        .await
        .map_err(|_| AuthError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to invalidate session".to_string(),
        })?;

    Ok(Json(LogoutResponsePayload { success: true }))
}

#[derive(Serialize)]
struct RefreshResponsePayload {
    token: Token,
}

async fn refresh(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
) -> Result<impl IntoResponse, AuthError> {
    if session.token.created_at + chrono::Duration::days(3) > chrono::Utc::now() {
        return Err(AuthError {
            status_code: StatusCode::FORBIDDEN,
            message: "Failed to refresh session".to_string(),
        });
    }

    sqlx::query("UPDATE sessions SET is_active = false WHERE id = $1")
        .bind(&session.token.id)
        .execute(&state.db)
        .await
        .map_err(|_| AuthError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to invalidate session".to_string(),
        })?;

    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let expires_at = chrono::Utc::now() + chrono::Duration::days(30);

    let (created_at,): (chrono::DateTime<chrono::Utc>,) = sqlx::query_as("INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING created_at;")
        .bind(&session.user.id)
        .bind(&token)
        .bind(&expires_at)
        .fetch_one(&state.db)
        .await
        .map_err(|_| AuthError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to issue session".to_string(),
        })?;

    Ok(Json(RefreshResponsePayload {
        token: Token {
            token,
            created_at,
            expires_at,
        },
    }))
}
