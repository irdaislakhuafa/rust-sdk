// memory backend (port of go-sdk/caches/memory.go)

use crate::caches::{Cache, Item, now_secs};
use crate::codes::Code;
use crate::errors;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct MemoryCache<T> {
    storage: RwLock<HashMap<String, Item<T>>>,
}

impl<T> MemoryCache<T> {
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

impl<T> Default for MemoryCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Cache<T> for MemoryCache<T>
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
        {
            let guard = self.storage.read().unwrap_or_else(|e| e.into_inner());
            if let Some(item) = guard.get(key)
                && item.ttl > now_secs()
            {
                return Ok(item.data.clone());
            }
        }

        {
            let mut guard = self.storage.write().unwrap_or_else(|e| e.into_inner());
            if let Some(item) = guard.get(key)
                && item.ttl <= now_secs()
            {
                guard.remove(key);
            }
        }

        let data = callback()?;
        let item = Item {
            ttl: now_secs() + ttl_secs,
            data: data.clone(),
        };
        let mut guard = self.storage.write().unwrap_or_else(|e| e.into_inner());
        guard.insert(key.to_string(), item);
        Ok(data)
    }

    fn clear(&self) {
        let mut guard = self.storage.write().unwrap_or_else(|e| e.into_inner());
        guard.clear();
    }

    fn forget(&self, key: &str) -> Result<T, crate::errors::SdkError> {
        let mut guard = self.storage.write().unwrap_or_else(|e| e.into_inner());
        if let Some(item) = guard.remove(key) {
            Ok(item.data)
        } else {
            Err(errors::new_with_code(
                Code::CacheKeyNotFound,
                "key not found",
            ))
        }
    }

    fn forget_fn(
        &self,
        f: &mut dyn FnMut(&str) -> Result<T, crate::errors::SdkError>,
    ) -> Result<Option<T>, crate::errors::SdkError>
    {
        let keys: Vec<String> = self.keys();
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
        let guard = self.storage.read().unwrap_or_else(|e| e.into_inner());
        guard.len() as u64
    }

    fn get(&self, key: &str) -> Result<T, crate::errors::SdkError> {
        let guard = self.storage.read().unwrap_or_else(|e| e.into_inner());
        match guard.get(key) {
            Some(item) => Ok(item.data.clone()),
            None => Err(errors::new_with_code(
                Code::CacheKeyNotFound,
                format!("cache with key '{}' not found", key),
            )),
        }
    }

    fn keys(&self) -> Vec<String> {
        let guard = self.storage.read().unwrap_or_else(|e| e.into_inner());
        guard.keys().cloned().collect()
    }
}


