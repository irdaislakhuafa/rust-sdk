// storage types and trait (port of go-sdk/storage)

use std::io::Read;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Minio,
    Disk,
    Dummy,
}

impl StorageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageType::Minio => "minio",
            StorageType::Disk => "disk",
            StorageType::Dummy => "dummy",
        }
    }
}

pub trait Storage: Send + Sync {
    fn put(&self, _ctx: (), params: PutParams) -> Result<PutResult, crate::errors::SdkError>;
    fn del(&self, _ctx: (), params: DelParams) -> Result<(), crate::errors::SdkError>;
    fn url(&self, _ctx: (), params: UrlParams) -> Result<UrlResult, crate::errors::SdkError>;
}

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_type: StorageType,
    pub base_dir: String,
    pub host: String,
    pub port: String,
    pub ssl: bool,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub token: String,
    pub region: String,
    pub max_retries: i32,
}

impl Config {
    pub fn parse_default(&mut self) {
        if self.max_retries <= 0 {
            self.max_retries = 1;
        }
    }
}

#[derive(Default)]
pub struct PutParams {
    pub dir_name: String,
    pub file_name: String,
    pub file_buffer: Option<Box<dyn Read + Send>>,
    pub file_size: i64,
    pub content_type: String,
    pub content_encoding: String,
    pub user_tags: std::collections::HashMap<String, String>,
    pub user_metadata: std::collections::HashMap<String, String>,
}

impl std::fmt::Debug for PutParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PutParams")
            .field("dir_name", &self.dir_name)
            .finish()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PutResult {
    pub dir_name: String,
    pub file_name: String,
    pub full_path: String,
    pub region: String,
    pub tag: String,
    pub size: i64,
}

#[derive(Debug, Clone, Default)]
pub struct DelParams {
    pub full_path: String,
    pub force: bool,
}

#[derive(Debug, Clone, Default)]
pub struct UrlParams {
    pub full_path: String,
    pub expire_duration: Duration,
}

impl UrlParams {
    pub fn parse_default(&mut self) {
        if self.expire_duration.as_millis() <= 0 {
            self.expire_duration = Duration::from_secs(24 * 3600);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UrlResult {
    pub scheme: String,
    pub host: String,
    pub full_path: String,
    pub full_url: String,
    pub raw_query: String,
}
