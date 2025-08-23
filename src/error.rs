use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::domain::errors::DomainError;

#[derive(Debug)]
pub enum AppError {
    Domain(DomainError),
    BadRequest(String),
    Internal(String),
}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        Self::Domain(value)
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Domain(DomainError::NotFound) => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody {
                    error: "not found".into(),
                }),
            )
                .into_response(),
            AppError::Domain(DomainError::Validation(msg)) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody { error: msg }),
            )
                .into_response(),
            AppError::Domain(DomainError::Conflict(msg)) => (
                StatusCode::CONFLICT,
                Json(ErrorBody { error: msg }),
            )
                .into_response(),
            AppError::Domain(DomainError::Internal) => {
                tracing::error!("internal domain error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorBody {
                        error: "internal server error".into(),
                    }),
                )
                    .into_response()
            }
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody { error: msg }),
            )
                .into_response(),
            AppError::Internal(msg) => {
                tracing::error!(%msg, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorBody {
                        error: "internal server error".into(),
                    }),
                )
                    .into_response()
            }
        }
    }
}

