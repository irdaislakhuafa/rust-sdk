// shared logger logic

use crate::appcontext;
use crate::appcontext::Interface;
use crate::log::config::FatalBehavior;
use crate::log::format;
use crate::log::level::Level;
use crate::log::message::LogMessage;
use crate::log::sink::LogSink;
use serde_json::{Map, Value};
use std::sync::{Arc, Mutex};

pub struct BaseLogger {
    pub level: Level,
    pub skip_frame_count: usize,
    pub sink: Box<dyn LogSink>,
    pub fatal_behavior: FatalBehavior,
    pub func_ctx_fields: Arc<Mutex<Option<crate::log::CtxFieldsFn>>>,
    pub is_merge: Arc<Mutex<bool>>,
}

impl BaseLogger {
    pub fn context_fields(&self, ctx: &crate::appcontext::AppContext) -> Map<String, Value> {
        let base = format::base_context_fields(ctx);
        let provider = self.func_ctx_fields.lock().ok().and_then(|g| g.clone());
        match provider {
            None => base,
            Some(f) => {
                let custom = f(ctx);
                let merge = *self.is_merge.lock().unwrap_or_else(|e| e.into_inner());
                if merge {
                    let mut out = base;
                    for (k, v) in custom {
                        out.insert(k, v);
                    }
                    out
                } else {
                    custom
                }
            }
        }
    }

    pub fn emit(&self, lvl: Level, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage) {
        if lvl < self.level {
            return;
        }
        let message = crate::log::message::get_caller(msg);
        let caller = msg.caller();
        let fields = self.context_fields(ctx);
        let line = format::build_record(lvl, &message, caller.as_deref(), fields);
        let mut bytes = line.into_bytes();
        bytes.push(b'\n');
        let _ = self.sink.write(&bytes);

        if lvl == Level::Fatal && self.fatal_behavior == FatalBehavior::Exit {
            std::process::exit(1);
        }
    }
}
