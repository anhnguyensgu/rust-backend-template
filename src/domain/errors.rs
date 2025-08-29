use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("internal error")]
    Internal,
}

impl IntoResponse for DomainError{
    fn into_response(self) -> Response {
        match self {
            DomainError::NotFound => (axum::http::StatusCode::NOT_FOUND, "Not Found".into()),
            DomainError::Validation(_) => (axum::http::StatusCode::BAD_REQUEST, self.to_string()),
            DomainError::Conflict(_) => (axum::http::StatusCode::CONFLICT, self.to_string()),
            DomainError::Internal => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }.into_response()

    }
}

