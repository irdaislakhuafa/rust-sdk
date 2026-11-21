// sqlite backend (port of go-sdk/caches/sqlite.go)

use crate::caches::{Cache, Item, now_secs};
use crate::codes::Code;
use crate::errors;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::SqlitePool;


pub struct SqliteCache<T> {
    pool: SqlitePool,
    rt: tokio::runtime::Runtime,
    table: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SqliteCache<T> {
    pub fn new(cfg: &crate::caches::Config) -> Result<Self, crate::errors::SdkError> {
        let dir = cfg.dir.clone().unwrap_or_else(|| ".".to_string());
        let db_path = format!("{}caches", dir);

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| errors::new_with_code(Code::SqlInit, e.to_string()))?;

        let pool = rt.block_on(async {
            sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect(&format!("sqlite:{}?mode=rwc", db_path))
                .await
        })
        .map_err(|e| errors::new_with_code(Code::SqlInit, e.to_string()))?;

        let table = "caches".to_string();
        let ddl = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                data TEXT NOT NULL
            )",
            table
        );
        rt.block_on(async {
            sqlx::query(&ddl).execute(&pool).await
        })
        .map_err(|e| errors::new_with_code(Code::SqlInit, e.to_string()))?;

        Ok(Self {
            pool,
            rt,
            table,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<T> SqliteCache<T>
where
    T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn sql_get(&self, key: &str) -> Result<(i64, String), crate::errors::SdkError> {
        let sql = format!("SELECT id, data FROM {} WHERE key = ?1", self.table);
        let res: Result<(i64, String), sqlx::Error> = self.rt.block_on(async {
            sqlx::query_as(&sql).bind(key).fetch_one(&self.pool).await
        });
        match res {
            Ok(row) => Ok(row),
            Err(sqlx::Error::RowNotFound) => Err(errors::new_with_code(
                Code::CacheKeyNotFound,
                format!("cache key '{}' not found", key),
            )),
            Err(e) => Err(errors::new_with_code(Code::CacheKeyNotFound, e.to_string())),
        }
    }

    fn sql_add(&self, key: &str, data: &str) -> Result<(), crate::errors::SdkError> {
        let sql = format!("INSERT INTO {} (key, data) VALUES (?1, ?2)", self.table);
        self.rt
            .block_on(async {
                sqlx::query(&sql).bind(key).bind(data).execute(&self.pool).await
            })
            .map_err(|e| errors::new_with_code(Code::CacheAddErr, e.to_string()))?;
        Ok(())
    }

    fn sql_update(&self, key: &str, data: &str) -> Result<(), crate::errors::SdkError> {
        let sql = format!("UPDATE {} SET data = ?1 WHERE key = ?2", self.table);
        self.rt
            .block_on(async {
                sqlx::query(&sql).bind(data).bind(key).execute(&self.pool).await
            })
            .map_err(|e| errors::new_with_code(Code::CacheAddErr, e.to_string()))?;
        Ok(())
    }

    fn sql_del(&self, key: &str) -> Result<(), crate::errors::SdkError> {
        let sql = format!("DELETE FROM {} WHERE key = ?1", self.table);
        self.rt
            .block_on(async {
                sqlx::query(&sql).bind(key).execute(&self.pool).await
            })
            .map_err(|e| errors::new_with_code(Code::CacheDelErr, e.to_string()))?;
        Ok(())
    }
}

impl<T> Cache<T> for SqliteCache<T>
where
    T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn remember(
        &self,
        key: &str,
        ttl_secs: u64,
        callback: &mut dyn FnMut() -> Result<T, crate::errors::SdkError>,
    ) -> Result<T, crate::errors::SdkError> {
        let now = now_secs();

        match self.sql_get(key) {
            Ok((_id, data_str)) => {
                let item: Item<T> = serde_json::from_str(&data_str).map_err(|e| {
                    errors::new_with_code(Code::CacheReadErr, e.to_string())
                })?;

                if item.ttl > now {
                    return Ok(item.data);
                }

                let data = callback()?;
                let new_item = Item {
                    ttl: now + ttl_secs,
                    data: data.clone(),
                };
                let json = serde_json::to_string(&new_item).map_err(|e| {
                    errors::new_with_code(Code::CacheAddErr, e.to_string())
                })?;
                self.sql_update(key, &json)?;
                Ok(data)
            }
            Err(e) => {
                let code = errors::get_code(e.as_ref());
                if code != Code::CacheKeyNotFound {
                    return Err(e);
                }
                let data = callback()?;
                let new_item = Item {
                    ttl: now + ttl_secs,
                    data: data.clone(),
                };
                let json = serde_json::to_string(&new_item).map_err(|e| {
                    errors::new_with_code(Code::CacheAddErr, e.to_string())
                })?;
                self.sql_add(key, &json)?;
                Ok(data)
            }
        }
    }

    fn clear(&self) {
        let sql = format!("DELETE FROM {}", self.table);
        let _ = self.rt.block_on(async { sqlx::query(&sql).execute(&self.pool).await });
    }

    fn forget(&self, key: &str) -> Result<T, crate::errors::SdkError> {
        let (_id, data_str) = self.sql_get(key)?;
        let item: Item<T> = serde_json::from_str(&data_str).map_err(|e| {
            errors::new_with_code(Code::CacheReadErr, e.to_string())
        })?;
        self.sql_del(key)?;
        Ok(item.data)
    }

    fn forget_fn(
        &self,
        f: &mut dyn FnMut(&str) -> Result<T, crate::errors::SdkError>,
    ) -> Result<Option<T>, crate::errors::SdkError> {
        let sql = format!("SELECT key FROM {}", self.table);
        let rows: Vec<(String,)> = self
            .rt
            .block_on(async { sqlx::query_as(&sql).fetch_all(&self.pool).await })
            .map_err(|e| errors::new_with_code(Code::CacheReadErr, e.to_string()))?;

        let keys: Vec<String> = rows.into_iter().map(|r| r.0).collect();
        let mut res: Option<T> = None;
        for k in &keys {
            match f(k) {
                Ok(v) => {
                    res = Some(v);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(res)
    }

    fn length(&self) -> u64 {
        let sql = format!("SELECT COUNT(*) FROM {}", self.table);
        let count: (i64,) = self
            .rt
            .block_on(async { sqlx::query_as(&sql).fetch_one(&self.pool).await })
            .unwrap_or((0,));
        count.0 as u64
    }

    fn get(&self, key: &str) -> Result<T, crate::errors::SdkError> {
        let (_id, data_str) = self.sql_get(key)?;
        let item: Item<T> = serde_json::from_str(&data_str).map_err(|e| {
            errors::new_with_code(Code::CacheReadErr, e.to_string())
        })?;
        let now = now_secs();
        if item.ttl <= now {
            return Err(errors::new_with_code(
                Code::CacheKeyNotFound,
                format!("cache key '{}' expired", key),
            ));
        }
        Ok(item.data)
    }

    fn keys(&self) -> Vec<String> {
        let sql = format!("SELECT key FROM {}", self.table);
        let rows: Vec<(String,)> = self
            .rt
            .block_on(async { sqlx::query_as(&sql).fetch_all(&self.pool).await })
            .unwrap_or_default();
        rows.into_iter().map(|r| r.0).collect()
    }
}