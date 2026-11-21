// log config (port of go-sdk/log/log.go Config)

use super::level::Level;
use super::storage::{OutputConfig, StorageDriver, StorageOpt};

pub const DEFAULT_SKIP_FRAME_COUNT: usize = 3;
pub const DEFAULT_MAX_AGE: i32 = 30;
pub const DEFAULT_MAX_BACKUPS: i32 = 3;
pub const DEFAULT_MAX_SIZE: i32 = 100;
pub const DEFAULT_FILE_LOCATION: &str = "logs/current.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FatalBehavior {
    #[default]
    LogOnly,
    Exit,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub level: Level,
    pub skip_frame_count: usize,
    pub storage: StorageOpt,
    pub fatal_behavior: FatalBehavior,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: Level::Info,
            skip_frame_count: 0,
            storage: StorageOpt::default(),
            fatal_behavior: FatalBehavior::default(),
        }
    }
}

impl Config {
    pub fn parse_default(&mut self) {
        if self.storage.file_location.is_empty() && self.storage.driver == StorageDriver::File {
            self.storage.file_location = DEFAULT_FILE_LOCATION.to_string();
        }
        // normalize driver string is handled at construction; driver already enum.
        if self.skip_frame_count == 0 {
            self.skip_frame_count = DEFAULT_SKIP_FRAME_COUNT;
        }
        if self.storage.driver == StorageDriver::File && self.storage.rotation.enable {
            if self.storage.rotation.max_age == 0 {
                self.storage.rotation.max_age = DEFAULT_MAX_AGE;
            }
            if self.storage.rotation.max_size == 0 {
                self.storage.rotation.max_size = DEFAULT_MAX_SIZE;
            }
            if self.storage.rotation.max_backups == 0 {
                self.storage.rotation.max_backups = DEFAULT_MAX_BACKUPS;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiConfig {
    pub level: Level,
    pub skip_frame_count: usize,
    pub fatal_behavior: FatalBehavior,
    pub outputs: Vec<OutputConfig>,
}

impl Default for MultiConfig {
    fn default() -> Self {
        Self {
            level: Level::Info,
            skip_frame_count: DEFAULT_SKIP_FRAME_COUNT,
            fatal_behavior: FatalBehavior::LogOnly,
            outputs: vec![OutputConfig::default()],
        }
    }
}
