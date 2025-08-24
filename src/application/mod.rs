use std::sync::Arc;

// Shared application state injected into handlers
#[derive(Clone)]
pub struct AppState<R> {
    pub user_repo: Arc<R>,
}
