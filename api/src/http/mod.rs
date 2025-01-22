use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub mod middleware;
pub mod rest;

pub struct HttpError {
    status_code: StatusCode,
    message: String,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct HttpErrorResponse<'a> {
            status_code: u16,
            message: &'a str,
        }

        let response = HttpErrorResponse {
            status_code: self.status_code.as_u16(),
            message: &self.message,
        };

        (self.status_code, Json(response)).into_response()
    }
}
