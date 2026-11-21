// log storage options (port of go-sdk/log/storage.go)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageDriver {
    Console,
    File,
}

impl StorageDriver {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_uppercase().as_str() {
            "FILE" => StorageDriver::File,
            _ => StorageDriver::Console,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            StorageDriver::Console => "CONSOLE",
            StorageDriver::File => "FILE",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StorageRotation {
    pub enable: bool,
    pub max_size: i32,
    pub max_backups: i32,
    pub max_age: i32,
    pub compress: bool,
}

#[derive(Debug, Clone)]
pub struct StorageOpt {
    pub driver: StorageDriver,
    pub file_location: String,
    pub rotation: StorageRotation,
}

impl Default for StorageOpt {
    fn default() -> Self {
        Self {
            driver: StorageDriver::Console,
            file_location: String::new(),
            rotation: StorageRotation::default(),
        }
    }
}

// OutputConfig: per-driver configuration for multi-config init.
use super::level::Level;

#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub driver: StorageDriver,
    pub level: Option<Level>,
    pub file_location: String,
    pub rotation: StorageRotation,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            driver: StorageDriver::Console,
            level: None,
            file_location: String::new(),
            rotation: StorageRotation::default(),
        }
    }
}
