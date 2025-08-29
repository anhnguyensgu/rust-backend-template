use std::sync::Arc;

use app::adapters::repository::in_memory::InMemoryUserRepo;
use rust_backend_template::{self as app, adapters::http::handlers::router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config and init tracing
    let cfg = app::config::AppConfig::from_env();
    app::telemetry::init_tracing();

    // Wire dependencies (outbound adapters)
    let repo = Arc::new(InMemoryUserRepo::new());
    // let user_service = app::application::user_service::UserService::new(repo);
    let state = app::application::AppState {
        user_repo: repo, // Inject the repository directly
    };

    let router = router(state);
    let addr = cfg.addr();
    tracing::info!(%addr, "listening on");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

