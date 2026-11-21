// log message normalization (port of go-sdk/log GetCaller behavior)

use crate::errors::SdkError;
use std::fmt;

pub trait LogMessage: Send + Sync {
    fn to_log_message(&self) -> String;
    fn caller(&self) -> Option<String> {
        None
    }
}

impl LogMessage for str {
    fn to_log_message(&self) -> String {
        self.to_string()
    }
}

impl LogMessage for String {
    fn to_log_message(&self) -> String {
        self.clone()
    }
}

impl LogMessage for &'static str {
    fn to_log_message(&self) -> String {
        (*self).to_string()
    }
}

impl LogMessage for SdkError {
    fn to_log_message(&self) -> String {
        self.to_string()
    }
    fn caller(&self) -> Option<String> {
        match crate::errors::get_caller(Some(self)) {
            Ok((file, line, msg)) => Some(format!("{}:{} --- {}", file, line, msg)),
            Err(_) => Some(self.to_string()),
        }
    }
}

// Helper for any Debug value.
pub struct DebugMsg<T>(pub T);

impl<T: fmt::Debug + Send + Sync> LogMessage for DebugMsg<T> {
    fn to_log_message(&self) -> String {
        format!("{:?}", self.0)
    }
}

// Convenience: implement for Box<dyn Error + Send + Sync> via wrapper not needed.

pub fn get_caller(msg: &dyn LogMessage) -> String {
    if let Some(c) = msg.caller() {
        return c;
    }
    let s = msg.to_log_message();
    // If the message itself is an error-like string, leave as is.
    s
}

// Helper to convert an error into a formatted caller string directly.
pub fn get_caller_from_error(err: &(dyn std::error::Error + Send + Sync + 'static)) -> String {
    // Try downcasting to SdkError not possible generically; fall back to formatting.
    // Attempt errors::get_caller via a boxed SdkError if it is one.
    let _ = get_caller;
    format!("{}", err)
}

// Expose new_with_code for impl modules convenience (re-export)
pub use crate::codes::Code as LogCode;
pub use crate::errors::new_with_code as new_err;
