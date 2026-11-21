// log module (port of go-sdk/log)

pub mod base;
pub mod config;
pub mod console;
pub mod file;
pub mod file_sink;
pub mod format;
pub mod level;
pub mod logger;
pub mod message;
pub mod multi;
pub mod sink;
pub mod storage;

#[cfg(test)]
mod log_test;

// Re-exports
pub use config::{
    Config, DEFAULT_FILE_LOCATION, DEFAULT_MAX_AGE, DEFAULT_MAX_BACKUPS, DEFAULT_MAX_SIZE,
    DEFAULT_SKIP_FRAME_COUNT, FatalBehavior, MultiConfig,
};
pub use console::ConsoleLogger;
pub use file::FileLogger;
pub use level::Level;
pub use logger::{CtxFieldsFn, Logger};
pub use message::{DebugMsg, LogMessage};
pub use multi::FanoutLogger;
pub use sink::{CompositeSink, LogSink, WriterSink, stderr_sink, stdout_sink};
pub use storage::{OutputConfig, StorageDriver, StorageOpt, StorageRotation};

use crate::errors::SdkError;

// Constants parity
pub const STORAGE_DRIVER_CONSOLE: &str = "CONSOLE";
pub const STORAGE_DRIVER_FILE: &str = "FILE";

pub const LEVEL_TRACE_VALUE: &str = "trace";
pub const LEVEL_DEBUG_VALUE: &str = "debug";
pub const LEVEL_INFO_VALUE: &str = "info";
pub const LEVEL_WARN_VALUE: &str = "warn";
pub const LEVEL_ERROR_VALUE: &str = "error";
pub const LEVEL_FATAL_VALUE: &str = "fatal";
pub const LEVEL_PANIC_VALUE: &str = "panic";

pub fn init(cfg: Config) -> Result<Box<dyn Logger>, SdkError> {
    let mut cfg = cfg;
    cfg.parse_default();
    match cfg.storage.driver {
        StorageDriver::Console => init_console(cfg),
        StorageDriver::File => init_file(cfg),
    }
}

pub fn init_console(cfg: Config) -> Result<Box<dyn Logger>, SdkError> {
    let mut cfg = cfg;
    cfg.parse_default();
    Ok(Box::new(ConsoleLogger::new(cfg)))
}

pub fn init_file(cfg: Config) -> Result<Box<dyn Logger>, SdkError> {
    let mut cfg = cfg;
    cfg.parse_default();
    Ok(Box::new(FileLogger::new(cfg)?))
}

pub fn init_multi(cfg: MultiConfig) -> Result<Box<dyn Logger>, SdkError> {
    let mut cfg = cfg;
    if cfg.outputs.is_empty() {
        cfg.outputs.push(OutputConfig::default());
    }
    if cfg.skip_frame_count == 0 {
        cfg.skip_frame_count = DEFAULT_SKIP_FRAME_COUNT;
    }

    let multiple = cfg.outputs.len() > 1;
    let mut children: Vec<Box<dyn Logger>> = Vec::new();

    for out in &cfg.outputs {
        let eff_level = out.level.unwrap_or(cfg.level);
        let eff_fatal = if multiple {
            FatalBehavior::LogOnly
        } else {
            cfg.fatal_behavior
        };
        let mut c = Config {
            level: eff_level,
            skip_frame_count: cfg.skip_frame_count,
            storage: StorageOpt {
                driver: out.driver,
                file_location: out.file_location.clone(),
                rotation: out.rotation.clone(),
            },
            fatal_behavior: eff_fatal,
        };
        c.parse_default();
        let logger = match out.driver {
            StorageDriver::Console => init_console(c)?,
            StorageDriver::File => init_file(c)?,
        };
        children.push(logger);
    }

    if children.len() == 1 {
        // apply requested fatal behavior if single
        // rebuild not needed; children[0] already has eff_fatal set correctly above
        Ok(children.into_iter().next().unwrap())
    } else {
        Ok(Box::new(FanoutLogger::new(children, cfg.fatal_behavior)))
    }
}

pub fn fanout(loggers: Vec<Box<dyn Logger>>, fatal_behavior: FatalBehavior) -> Box<dyn Logger> {
    Box::new(FanoutLogger::new(loggers, fatal_behavior))
}

pub fn init_with_sink(
    level: Level,
    sink: Box<dyn LogSink>,
    fatal_behavior: FatalBehavior,
) -> Box<dyn Logger> {
    // Reuse ConsoleLogger shell with custom sink.
    Box::new(crate::log::console::ConsoleLogger::with_sink(
        Config {
            level,
            skip_frame_count: DEFAULT_SKIP_FRAME_COUNT,
            storage: StorageOpt::default(),
            fatal_behavior,
        },
        sink,
    ))
}

// Go-style GetCaller: normalize an input into a formatted string.
pub fn get_caller(value: &dyn std::fmt::Debug) -> String {
    format!("{:?}", value)
}

pub fn get_caller_err(err: &(dyn std::error::Error + Send + Sync + 'static)) -> String {
    format!("{}", err)
}

// Helper to convert a &str driver into StorageDriver when constructing Config manually.
pub fn parse_driver(s: &str) -> StorageDriver {
    StorageDriver::parse(s)
}
