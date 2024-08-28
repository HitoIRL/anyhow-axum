use std::string::ToString;

use axum::{http::StatusCode, response::{IntoResponse, Response}};

use crate::Error;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            self.to_string(),
        )
        .into_response()
    }
}
