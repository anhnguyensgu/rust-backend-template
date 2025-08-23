use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    adapters::http::AppState,
    domain::ports::UserRepository,
    domain::user::NewUser,
    error::AppError,
};

use super::dto::{CreateUserRequest, UserResponse};

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn create_user<R: UserRepository + Clone>(
    State(state): State<AppState<R>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let new = NewUser {
        name: payload.name,
        email: payload.email,
    };
    let user = state.user_service.create_user(new).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

pub async fn get_user<R: UserRepository + Clone>(
    Path(id): Path<Uuid>,
    State(state): State<AppState<R>>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.get_user(id).await?;
    Ok(Json(UserResponse::from(user)))
}
