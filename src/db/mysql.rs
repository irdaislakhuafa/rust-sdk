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
        "3306"
    } else {
        cfg.port.as_str()
    };
    let name = if cfg.name.is_empty() {
        "mysql"
    } else {
        cfg.name.as_str()
    };
    // Note: Go's `parseTime` option is always-on in sqlx MySQL when decoding
    // into chrono types; kept in Config for parity but not encoded in the URL.
    format!("mysql://{}:{}@{}:{}/{}", cfg.user, cfg.password, host, port, name)
}

pub async fn connect(cfg: Config) -> Result<sqlx::AnyPool, SdkError> {
    let url = build_url(&cfg);
    sqlx::AnyPool::connect(&url)
        .await
        .map_err(|e| new_with_code(Code::SqlInit, e.to_string()))
}