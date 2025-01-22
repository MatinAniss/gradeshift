use axum::{
    body::Body,
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::AppState;

use super::HttpError;

#[derive(Clone)]
pub struct Session {
    pub token: Token,
    pub user: User,
}

#[derive(Clone)]
pub struct Token {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct User {
    pub id: Uuid,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, HttpError> {
    let auth_header = match req.headers().get(http::header::AUTHORIZATION) {
        Some(header) => header.to_str().map_err(|_| HttpError {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Missing authorization token".to_string(),
        })?,
        None => {
            return Err(HttpError {
                status_code: StatusCode::UNAUTHORIZED,
                message: "Missing authorization header".to_string(),
            });
        }
    };

    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());

    let (session_id, user_id, created_at, expires_at, is_active): (
        Uuid,
        Uuid,
        chrono::DateTime<chrono::Utc>,
        chrono::DateTime<chrono::Utc>,
        bool,
    ) = sqlx::query_as(
        "SELECT id, user_id, created_at, expires_at, is_active FROM sessions WHERE token = $1;",
    )
    .bind(&token)
    .fetch_one(&state.db)
    .await
    .map_err(|_| HttpError {
        status_code: StatusCode::UNAUTHORIZED,
        message: "Authorization token is invalid".to_string(),
    })?;

    if expires_at < chrono::Utc::now() || !is_active {
        Err(HttpError {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Authorization token is expired".to_string(),
        })
    } else {
        req.extensions_mut().insert(Session {
            token: Token {
                id: session_id,
                created_at: created_at,
            },
            user: User { id: user_id },
        });

        Ok(next.run(req).await)
    }
}
