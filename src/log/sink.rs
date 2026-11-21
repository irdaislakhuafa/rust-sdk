// log sink abstraction (output destination)

use crate::codes::Code;
use crate::errors::{SdkError, new_with_code};
use std::io::Write;
use std::sync::{Arc, Mutex};

pub trait LogSink: Send + Sync + std::fmt::Debug {
    fn write(&self, entry: &[u8]) -> Result<(), SdkError>;
    fn cleanup(&self) -> Result<(), SdkError> {
        Ok(())
    }
}

// A sink backed by any Write implementor (for tests / stdout).
pub struct WriterSink<W> {
    writer: Arc<Mutex<W>>,
}

impl<W> std::fmt::Debug for WriterSink<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WriterSink").finish()
    }
}

impl<W: Write + Send + 'static> WriterSink<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: Arc::new(Mutex::new(writer)),
        }
    }
}

impl<W: Write + Send + 'static> LogSink for WriterSink<W> {
    fn write(&self, entry: &[u8]) -> Result<(), SdkError> {
        let mut w = self.writer.lock().map_err(|_| {
            new_with_code(Code::LogWriteFailure, "failed to lock writer".to_string())
        })?;
        w.write_all(entry)
            .map_err(|e| new_with_code(Code::LogWriteFailure, format!("write failed: {}", e)))?;
        let _ = w.flush();
        Ok(())
    }
    fn cleanup(&self) -> Result<(), SdkError> {
        Ok(())
    }
}

// Convenience constructors
pub fn stdout_sink() -> Box<dyn LogSink> {
    Box::new(WriterSink::new(std::io::stdout()))
}

pub fn stderr_sink() -> Box<dyn LogSink> {
    Box::new(WriterSink::new(std::io::stderr()))
}

// CompositeSink: fan-out the same record to multiple sinks.
#[derive(Debug, Default)]
pub struct CompositeSink {
    sinks: Vec<Box<dyn LogSink>>,
}

impl CompositeSink {
    pub fn new(sinks: Vec<Box<dyn LogSink>>) -> Self {
        Self { sinks }
    }
    pub fn push(&mut self, sink: Box<dyn LogSink>) {
        self.sinks.push(sink);
    }
}

impl LogSink for CompositeSink {
    fn write(&self, entry: &[u8]) -> Result<(), SdkError> {
        let mut first_err: Option<SdkError> = None;
        for sink in &self.sinks {
            if let Err(e) = sink.write(entry) {
                if first_err.is_none() {
                    first_err = Some(e);
                }
            }
        }
        match first_err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
    fn cleanup(&self) -> Result<(), SdkError> {
        let mut first_err: Option<SdkError> = None;
        for sink in &self.sinks {
            if let Err(e) = sink.cleanup() {
                if first_err.is_none() {
                    first_err = Some(e);
                }
            }
        }
        match first_err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}
