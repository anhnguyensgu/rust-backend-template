use app::adapters::repository::in_memory::InMemoryUserRepo;
use axum::Router;
use rust_backend_template::{
    self as app, AppState, adapters::http::routing, domain::ports::UserRepository,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config and init tracing
    let cfg = app::config::AppConfig::from_env();
    app::telemetry::init_tracing();

    // Wire dependencies (outbound adapters)
    let repo = InMemoryUserRepo::new();
    let user_service = app::application::user_service::UserService::new(repo);
    let state = app::adapters::http::AppState {
        user_service: std::sync::Arc::new(user_service),
    };

    // Build router (inbound adapter)
    let router = router::<InMemoryUserRepo>(state);
    // Serve
    let addr = cfg.addr();
    tracing::info!(%addr, "listening on");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

pub fn router<R>(state: AppState<R>) -> Router
where
    R: UserRepository + Clone + 'static,
{
    routing::build::<R>().with_state(state)
}
