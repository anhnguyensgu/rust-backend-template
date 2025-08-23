use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("APP_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(3000);
        Self { host, port }
    }

    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("invalid APP_HOST/APP_PORT configuration")
    }
}

