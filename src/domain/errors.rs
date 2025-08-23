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

