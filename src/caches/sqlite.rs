// sqlite backend (port of go-sdk/caches/sqlite.go)

use crate::caches::{Cache, Item, now_secs};
use crate::codes::Code;
use crate::errors;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Mutex;

pub struct SqliteCache<T> {
    conn: Mutex<rusqlite::Connection>,
    table: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SqliteCache<T> {
    pub fn new(cfg: &crate::caches::Config) -> Result<Self, crate::errors::SdkError> {
        let dir = cfg.dir.clone().unwrap_or_else(|| ".".to_string());
        let db_path = format!("{}caches", dir);

        let conn = rusqlite::Connection::open(&db_path).map_err(|e| {
            errors::new_with_code(Code::SqlInit, e.to_string())
        })?;

        let table = "caches".to_string();
        let ddl = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                data TEXT NOT NULL
            )",
            table
        );
        conn.execute(&ddl, []).map_err(|e| {
            errors::new_with_code(Code::SqlInit, e.to_string())
        })?;

        Ok(Self {
            conn: Mutex::new(conn),
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
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let sql = format!("SELECT id, data FROM {} WHERE key = ?1", self.table);
        conn.query_row(&sql, rusqlite::params![key], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => errors::new_with_code(
                Code::CacheKeyNotFound,
                format!("cache key '{}' not found", key),
            ),
            _ => errors::new_with_code(Code::CacheKeyNotFound, e.to_string()),
        })
    }

    fn sql_add(&self, key: &str, data: &str) -> Result<(), crate::errors::SdkError> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let sql = format!("INSERT INTO {} (key, data) VALUES (?1, ?2)", self.table);
        conn.execute(&sql, rusqlite::params![key, data])
            .map_err(|e| errors::new_with_code(Code::CacheAddErr, e.to_string()))?;
        Ok(())
    }

    fn sql_update(&self, key: &str, data: &str) -> Result<(), crate::errors::SdkError> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let sql = format!("UPDATE {} SET data = ?1 WHERE key = ?2", self.table);
        conn.execute(&sql, rusqlite::params![data, key])
            .map_err(|e| errors::new_with_code(Code::CacheAddErr, e.to_string()))?;
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
    ) -> Result<T, crate::errors::SdkError>
    {
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
                let item = Item {
                    ttl: now + ttl_secs,
                    data: data.clone(),
                };
                let json = serde_json::to_string(&item).map_err(|e| {
                    errors::new_with_code(Code::CacheAddErr, e.to_string())
                })?;
                self.sql_add(key, &json)?;
                Ok(data)
            }
        }
    }

    fn clear(&self) {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let sql = format!("DELETE FROM {}", self.table);
        let _ = conn.execute(&sql, []);
    }

    fn forget(&self, key: &str) -> Result<T, crate::errors::SdkError> {
        let (_id, data_str) = self.sql_get(key)?;
        let item: Item<T> = serde_json::from_str(&data_str).map_err(|e| {
            errors::new_with_code(Code::CacheReadErr, e.to_string())
        })?;

        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let sql = format!("DELETE FROM {} WHERE key = ?1", self.table);
        let affected = conn
            .execute(&sql, rusqlite::params![key])
            .map_err(|e| errors::new_with_code(Code::CacheDelErr, e.to_string()))?;

        if affected == 0 {
            return Err(errors::new_with_code(
                Code::CacheKeyNotFound,
                format!("cache key '{}' not found", key),
            ));
        }
        Ok(item.data)
    }

    fn forget_fn(
        &self,
        f: &mut dyn FnMut(&str) -> Result<T, crate::errors::SdkError>,
    ) -> Result<Option<T>, crate::errors::SdkError>
    {
        let keys = self.keys();
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
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let sql = format!("SELECT COUNT(*) FROM {}", self.table);
        conn.query_row(&sql, [], |row| row.get::<_, i64>(0))
            .map(|v| v as u64)
            .unwrap_or(0)
    }

    fn get(&self, key: &str) -> Result<T, crate::errors::SdkError> {
        let (_id, data_str) = self.sql_get(key)?;
        let item: Item<T> = serde_json::from_str(&data_str).map_err(|e| {
            errors::new_with_code(Code::CacheReadErr, e.to_string())
        })?;
        Ok(item.data)
    }

    fn keys(&self) -> Vec<String> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let sql = format!("SELECT key FROM {}", self.table);
        let mut stmt = match conn.prepare(&sql) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let mut rows = match stmt.query([]) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let mut out = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            if let Ok(k) = row.get::<_, String>(0) {
                out.push(k);
            }
        }
        out
    }
}


