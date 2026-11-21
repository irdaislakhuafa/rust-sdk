// db module (port of go-sdk/db)
//
// Abstraction is `sqlx`'s `AnyPool` (closest Rust equivalent to Go's `*sql.DB`):
// driver-agnostic, pooled, async. This module provides config + init factory.

use crate::codes::Code;
use crate::errors::{new_with_code, SdkError};
use serde::{Deserialize, Serialize};
pub type DbPool = sqlx::AnyPool;

pub const DRIVER_MYSQL: &str = "mysql";
pub const DRIVER_POSTGRES: &str = "postgres";
pub const DRIVER_SQLITE: &str = "sqlite3";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Options {
    pub ssl_mode: String,
    pub parse_time: bool,
    pub in_memory: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub driver: String,
    pub user: String,
    pub password: String,
    pub port: String,
    pub host: String,
    pub name: String,
    pub options: Options,
}

pub mod mysql;
pub mod postgres;
pub mod sqlite;

static DRIVERS_INSTALLED: std::sync::Once = std::sync::Once::new();

fn install_drivers() {
    DRIVERS_INSTALLED.call_once(|| {
        sqlx::any::install_default_drivers();
    });
}

pub async fn init(cfg: Config) -> Result<sqlx::AnyPool, SdkError> {
    install_drivers();
    match cfg.driver.as_str() {
        DRIVER_MYSQL => mysql::connect(cfg).await,
        DRIVER_POSTGRES => postgres::connect(cfg).await,
        DRIVER_SQLITE => sqlite::connect(cfg).await,
        _ => Err(new_with_code(
            Code::NotImplemented,
            format!("driver name {} not implemented!", cfg.driver),
        )),
    }
}

pub async fn init_mysql(cfg: Config) -> Result<sqlx::AnyPool, SdkError> {
    install_drivers();
    mysql::connect(cfg).await
}

pub async fn init_postgres(cfg: Config) -> Result<sqlx::AnyPool, SdkError> {
    install_drivers();
    postgres::connect(cfg).await
}

pub async fn init_sqlite(cfg: Config) -> Result<sqlx::AnyPool, SdkError> {
    install_drivers();
    sqlite::connect(cfg).await
}

#[cfg(test)]
mod db_test;
#[cfg(test)]
mod sqlite_test;
#[cfg(test)]
mod mysql_test;
#[cfg(test)]
mod postgres_test;
