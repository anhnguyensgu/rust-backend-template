use axum::{routing::{get, post}, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use super::{handlers, AppState};
use crate::domain::ports::UserRepository;

pub fn build<R>() -> Router<AppState<R>>
where
    R: UserRepository + Clone + 'static,
{
    Router::new()
        .route("/health", get(handlers::health))
        .route("/users", post(handlers::create_user::<R>))
        .route("/users/{id}", get(handlers::get_user::<R>))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
