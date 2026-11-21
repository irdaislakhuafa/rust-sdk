use crate::codes::Code;
use crate::errors::{new_with_code, SdkError};
use crate::db::Config;

pub async fn connect(cfg: Config) -> Result<sqlx::AnyPool, SdkError> {
    let url = if cfg.options.in_memory {
        "sqlite::memory:?cache=shared".to_string()
    } else {
        format!("sqlite:{}.db?mode=rwc", cfg.name)
    };

    sqlx::AnyPool::connect(&url)
        .await
        .map_err(|e| new_with_code(Code::SqlInit, e.to_string()))
}