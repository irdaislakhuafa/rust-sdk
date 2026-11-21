// FanoutLogger: delegates logging to multiple independent loggers.

use crate::appcontext::AppContext;
use crate::log::config::FatalBehavior;
use crate::log::logger::{CtxFieldsFn, Logger};
use crate::log::message::LogMessage;

pub struct FanoutLogger {
    loggers: Vec<Box<dyn Logger>>,
    fatal_behavior: FatalBehavior,
}

impl FanoutLogger {
    pub fn new(loggers: Vec<Box<dyn Logger>>, fatal_behavior: FatalBehavior) -> Self {
        Self {
            loggers,
            fatal_behavior,
        }
    }
    pub fn push(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }
}

impl Logger for FanoutLogger {
    fn trace(&self, ctx: &AppContext, msg: &dyn LogMessage) {
        for l in &self.loggers {
            l.trace(ctx, msg);
        }
    }
    fn debug(&self, ctx: &AppContext, msg: &dyn LogMessage) {
        for l in &self.loggers {
            l.debug(ctx, msg);
        }
    }
    fn info(&self, ctx: &AppContext, msg: &dyn LogMessage) {
        for l in &self.loggers {
            l.info(ctx, msg);
        }
    }
    fn warn(&self, ctx: &AppContext, msg: &dyn LogMessage) {
        for l in &self.loggers {
            l.warn(ctx, msg);
        }
    }
    fn error(&self, ctx: &AppContext, msg: &dyn LogMessage) {
        for l in &self.loggers {
            l.error(ctx, msg);
        }
    }
    fn fatal(&self, ctx: &AppContext, msg: &dyn LogMessage) {
        for l in &self.loggers {
            l.fatal(ctx, msg);
        }
        if self.fatal_behavior == FatalBehavior::Exit {
            std::process::exit(1);
        }
    }

    fn with_ctx_fields(&mut self, f: Option<CtxFieldsFn>) -> &mut dyn Logger {
        for l in &mut self.loggers {
            l.with_ctx_fields(f.clone());
        }
        self
    }
    fn merged_fields(&mut self) -> &mut dyn Logger {
        for l in &mut self.loggers {
            l.merged_fields();
        }
        self
    }
    fn cleanup(&self) {
        for l in &self.loggers {
            l.cleanup();
        }
    }
}
