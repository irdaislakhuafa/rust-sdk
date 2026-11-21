use crate::log::base::BaseLogger;
use crate::log::config::Config;
use crate::log::file_sink::{FileSink, SizeRotatePolicy};
use crate::log::level::Level;
use crate::log::logger::Logger;
use crate::log::message::LogMessage;
use crate::log::sink::LogSink;
use std::sync::Arc;

pub struct FileLogger {
    base: BaseLogger,
}

impl FileLogger {
    pub fn new(cfg: Config) -> Result<Self, crate::errors::SdkError> {
        let path = if cfg.storage.file_location.is_empty() {
            crate::log::config::DEFAULT_FILE_LOCATION.to_string()
        } else {
            cfg.storage.file_location.clone()
        };
        let policy = if cfg.storage.rotation.enable {
            let max_bytes = (cfg.storage.rotation.max_size as u64) * 1024 * 1024;
            let max_backups = if cfg.storage.rotation.max_backups <= 0 {
                3
            } else {
                cfg.storage.rotation.max_backups as usize
            };
            Some(Arc::new(SizeRotatePolicy {
                max_size_bytes: if max_bytes == 0 {
                    (crate::log::config::DEFAULT_MAX_SIZE as u64) * 1024 * 1024
                } else {
                    max_bytes
                },
                max_backups,
            })
                as Arc<dyn crate::log::file_sink::RotatePolicy>)
        } else {
            None
        };
        let sink = FileSink::new(&path, policy)?;
        Ok(Self {
            base: BaseLogger {
                level: cfg.level,
                skip_frame_count: cfg.skip_frame_count,
                sink: Box::new(sink),
                fatal_behavior: cfg.fatal_behavior,
                func_ctx_fields: Arc::new(std::sync::Mutex::new(None)),
                is_merge: Arc::new(std::sync::Mutex::new(false)),
            },
        })
    }

    pub fn with_sink(cfg: Config, sink: Box<dyn LogSink>) -> Self {
        Self {
            base: BaseLogger {
                level: cfg.level,
                skip_frame_count: cfg.skip_frame_count,
                sink,
                fatal_behavior: cfg.fatal_behavior,
                func_ctx_fields: Arc::new(std::sync::Mutex::new(None)),
                is_merge: Arc::new(std::sync::Mutex::new(false)),
            },
        }
    }
}

impl Logger for FileLogger {
    fn trace(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage) {
        self.base.emit(Level::Trace, ctx, msg);
    }
    fn debug(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage) {
        self.base.emit(Level::Debug, ctx, msg);
    }
    fn info(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage) {
        self.base.emit(Level::Info, ctx, msg);
    }
    fn warn(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage) {
        self.base.emit(Level::Warn, ctx, msg);
    }
    fn error(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage) {
        self.base.emit(Level::Error, ctx, msg);
    }
    fn fatal(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage) {
        self.base.emit(Level::Fatal, ctx, msg);
    }

    fn with_ctx_fields(&mut self, f: Option<crate::log::CtxFieldsFn>) -> &mut dyn Logger {
        if let Ok(mut g) = self.base.func_ctx_fields.lock() {
            *g = f;
        }
        self
    }
    fn merged_fields(&mut self) -> &mut dyn Logger {
        if let Ok(mut g) = self.base.is_merge.lock() {
            *g = true;
        }
        self
    }
    fn cleanup(&self) {
        let _ = self.base.sink.cleanup();
    }
}
