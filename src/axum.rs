use std::string::{String, ToString};

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: self.to_string() }),
        )
        .into_response()
    }
}
