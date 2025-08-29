use crate::{
    adapters::http::{claim::Claims, dto::UserResponse},
    application::AppState,
    domain::{ports::UserRepository, user::NewUser},
    error::AppError,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use tracing::info;
use uuid::Uuid;

pub async fn get_user<R: UserRepository + Clone + Send + Sync + 'static>(
    Path(id): Path<Uuid>,
    State(state): State<AppState<R>>,
    _claims: Claims,
) -> Result<impl IntoResponse, AppError> {
    info!("Handler get_user called with id: {id}");
    let _user = state.user_repo.get_user(id).await?;
    Ok(Json(UserResponse {
        id,
        ..Default::default()
    }))
}

pub async fn create_user<R: UserRepository>(
    State(state): State<AppState<R>>,
) -> Result<impl IntoResponse, AppError> {
    state.user_repo.create(NewUser::default()).await?;
    Ok(Json(UserResponse {
        id: Uuid::new_v4(),
        ..Default::default()
    }))
}

pub fn router<T: UserRepository + Clone + Send + Sync + 'static>(state: AppState<T>) -> Router {
    Router::new()
        .route("/{id}", get(get_user))
        .route("/", post(get_user))
        .with_state(state)
}
