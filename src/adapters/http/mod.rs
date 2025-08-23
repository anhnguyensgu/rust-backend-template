pub mod dto;
pub mod handlers;
pub mod routing;

use axum::Router;

use crate::{application::user_service::UserService, domain::ports::UserRepository};

#[derive(Clone)]
pub struct AppState<R: UserRepository + Clone> {
    pub user_service: std::sync::Arc<UserService<R>>,
}

pub fn router<R>() -> Router<AppState<R>>
where
    R: UserRepository + Clone + 'static,
{
    routing::build::<R>()
}
