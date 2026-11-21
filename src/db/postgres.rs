use crate::codes::Code;
use crate::errors::{new_with_code, SdkError};
use crate::db::Config;

pub fn build_url(cfg: &Config) -> String {
    let host = if cfg.host.is_empty() {
        "localhost"
    } else {
        cfg.host.as_str()
    };
    let port = if cfg.port.is_empty() {
        "5432"
    } else {
        cfg.port.as_str()
    };
    let name = if cfg.name.is_empty() {
        "postgres"
    } else {
        cfg.name.as_str()
    };
    let mut url = format!(
        "postgres://{}:{}@{}:{}/{}",
        cfg.user, cfg.password, host, port, name
    );
    if !cfg.options.ssl_mode.is_empty() {
        url.push_str(&format!("?sslmode={}", cfg.options.ssl_mode));
    }
    url
}

pub async fn connect(cfg: Config) -> Result<sqlx::AnyPool, SdkError> {
    let url = build_url(&cfg);
    sqlx::AnyPool::connect(&url)
        .await
        .map_err(|e| new_with_code(Code::SqlInit, e.to_string()))
}