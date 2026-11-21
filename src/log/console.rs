use crate::log::base::BaseLogger;
use crate::log::config::Config;
use crate::log::level::Level;
use crate::log::logger::Logger;
use crate::log::message::LogMessage;
use crate::log::sink::{LogSink, stdout_sink};

pub struct ConsoleLogger {
    base: BaseLogger,
}

impl ConsoleLogger {
    pub fn new(cfg: Config) -> Self {
        Self {
            base: BaseLogger {
                level: cfg.level,
                skip_frame_count: cfg.skip_frame_count,
                sink: stdout_sink(),
                fatal_behavior: cfg.fatal_behavior,
                func_ctx_fields: std::sync::Arc::new(std::sync::Mutex::new(None)),
                is_merge: std::sync::Arc::new(std::sync::Mutex::new(false)),
            },
        }
    }

    pub fn with_sink(cfg: Config, sink: Box<dyn LogSink>) -> Self {
        Self {
            base: BaseLogger {
                level: cfg.level,
                skip_frame_count: cfg.skip_frame_count,
                sink,
                fatal_behavior: cfg.fatal_behavior,
                func_ctx_fields: std::sync::Arc::new(std::sync::Mutex::new(None)),
                is_merge: std::sync::Arc::new(std::sync::Mutex::new(false)),
            },
        }
    }
}

impl Logger for ConsoleLogger {
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
