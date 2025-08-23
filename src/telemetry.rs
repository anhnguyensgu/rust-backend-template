use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() {
    let default_filter = "info,tower_http=info";
    let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| default_filter.to_string());
    let _ = fmt()
        .with_env_filter(EnvFilter::new(env_filter))
        .with_target(false)
        .compact()
        .try_init();
}

