// Logger trait (app-facing contract)

use crate::appcontext;
use crate::log::message::LogMessage;

pub type CtxFieldsFn = std::sync::Arc<
    dyn Fn(&crate::appcontext::AppContext) -> serde_json::Map<String, serde_json::Value>
        + Send
        + Sync,
>;

pub trait Logger: Send + Sync {
    fn trace(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage);
    fn debug(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage);
    fn info(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage);
    fn warn(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage);
    fn error(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage);
    fn fatal(&self, ctx: &crate::appcontext::AppContext, msg: &dyn LogMessage);

    fn with_ctx_fields(&mut self, f: Option<CtxFieldsFn>) -> &mut dyn Logger;
    fn merged_fields(&mut self) -> &mut dyn Logger;
    fn cleanup(&self);
}
