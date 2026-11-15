// caches module (port of go-sdk/caches)
//
// Shared types/helpers live here; backend implementations are split into
// `memory.rs` and `sqlite.rs` (mirroring go-sdk's `memory.go`/`sqlite.go`).
// The public abstraction is the `Cache<T>` trait (mirroring Go's `Interface[T]`).

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod memory;
pub mod sqlite;

#[cfg(test)]
mod memory_test;
#[cfg(test)]
mod sqlite_test;

pub use memory::MemoryCache;
pub use sqlite::SqliteCache;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Memory,
    Sqlite,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_type: StorageType,
    /// Directory/prefix used by the SQLite backend (Go: `Config.Dir`).
    pub dir: Option<String>,
}

impl Config {
    pub fn memory() -> Self {
        Self {
            storage_type: StorageType::Memory,
            dir: None,
        }
    }

    pub fn sqlite(dir: impl Into<String>) -> Self {
        Self {
            storage_type: StorageType::Sqlite,
            dir: Some(dir.into()),
        }
    }
}

/// Go: `item[T] { TTL uint64; Data T }`
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Item<T> {
    pub(crate) ttl: u64,
    pub(crate) data: T,
}

pub(crate) fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Abstract cache interface (mirrors Go's `Interface[T]`).
pub trait Cache<T>: Send + Sync
where
    T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn remember(
        &self,
        key: &str,
        ttl_secs: u64,
        callback: &mut dyn FnMut() -> Result<T, crate::errors::SdkError>,
    ) -> Result<T, crate::errors::SdkError>;

    fn clear(&self);

    fn forget(&self, key: &str) -> Result<T, crate::errors::SdkError>;

    fn forget_fn(
        &self,
        f: &mut dyn FnMut(&str) -> Result<T, crate::errors::SdkError>,
    ) -> Result<Option<T>, crate::errors::SdkError>;

    fn length(&self) -> u64;

    fn get(&self, key: &str) -> Result<T, crate::errors::SdkError>;

    fn keys(&self) -> Vec<String>;
}

/// Factory mirroring Go's `Init[T](cfg)`.
pub fn init<T>(cfg: Config) -> Result<Box<dyn Cache<T>>, crate::errors::SdkError>
where
    T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    match cfg.storage_type {
        StorageType::Memory => Ok(Box::new(MemoryCache::new())),
        StorageType::Sqlite => {
            let sc = SqliteCache::<T>::new(&cfg)?;
            Ok(Box::new(sc))
        }
    }
}
