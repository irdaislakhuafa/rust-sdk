// file sink with in-house minimal size-based rotation

use crate::codes::Code;
use crate::errors::{SdkError, new_with_code};
use crate::log::sink::LogSink;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct RotatePolicyCfg {
    pub max_size_bytes: u64,
    pub max_backups: usize,
}

pub trait RotatePolicy: Send + Sync + std::fmt::Debug {
    fn should_rotate(&self, current_size: u64, next_len: u64) -> bool;
    fn rotate(&self, path: &Path) -> Result<(), SdkError>;
}

#[derive(Debug)]
pub struct SizeRotatePolicy {
    pub max_size_bytes: u64,
    pub max_backups: usize,
}

impl RotatePolicy for SizeRotatePolicy {
    fn should_rotate(&self, current_size: u64, next_len: u64) -> bool {
        self.max_size_bytes > 0 && current_size + next_len > self.max_size_bytes
    }
    fn rotate(&self, path: &Path) -> Result<(), SdkError> {
        // shift backups: .1 -> .2 ... then current -> .1
        for i in (1..self.max_backups).rev() {
            let from = backup_path(path, i);
            let to = backup_path(path, i + 1);
            if from.exists() {
                let _ = fs::rename(&from, &to);
            }
        }
        if path.exists() {
            let to = backup_path(path, 1);
            fs::rename(path, &to).map_err(|e| {
                new_with_code(
                    Code::LogWriteFailure,
                    format!("rotate rename failed: {}", e),
                )
            })?;
        }
        Ok(())
    }
}

fn backup_path(path: &Path, n: usize) -> PathBuf {
    let mut s = path.as_os_str().to_os_string();
    s.push(format!(".{}", n));
    PathBuf::from(s)
}

#[derive(Debug)]
pub struct FileSink {
    path: PathBuf,
    policy: Option<Arc<dyn RotatePolicy>>,
    inner: Arc<Mutex<std::fs::File>>,
}

impl FileSink {
    pub fn new(
        path: impl AsRef<Path>,
        policy: Option<Arc<dyn RotatePolicy>>,
    ) -> Result<Self, SdkError> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| {
                    new_with_code(Code::LogInitFailure, format!("create dir failed: {}", e))
                })?;
            }
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| new_with_code(Code::LogInitFailure, format!("open file failed: {}", e)))?;
        Ok(Self {
            path,
            policy,
            inner: Arc::new(Mutex::new(file)),
        })
    }
}

impl LogSink for FileSink {
    fn write(&self, entry: &[u8]) -> Result<(), SdkError> {
        // rotate if needed
        if let Some(policy) = &self.policy {
            let current_size = fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0);
            if policy.should_rotate(current_size, entry.len() as u64) {
                // close, rotate, reopen
                {
                    let mut f = self.inner.lock().map_err(|_| {
                        new_with_code(Code::LogWriteFailure, "lock failed".to_string())
                    })?;
                    let _ = f.flush();
                }
                policy.rotate(&self.path)?;
                let new_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.path)
                    .map_err(|e| {
                        new_with_code(Code::LogInitFailure, format!("reopen failed: {}", e))
                    })?;
                let mut f = self
                    .inner
                    .lock()
                    .map_err(|_| new_with_code(Code::LogWriteFailure, "lock failed".to_string()))?;
                *f = new_file;
            }
        }
        let mut f = self
            .inner
            .lock()
            .map_err(|_| new_with_code(Code::LogWriteFailure, "lock failed".to_string()))?;
        f.write_all(entry)
            .map_err(|e| new_with_code(Code::LogWriteFailure, format!("write failed: {}", e)))?;
        let _ = f.flush();
        Ok(())
    }
    fn cleanup(&self) -> Result<(), SdkError> {
        // best-effort flush/close by dropping handle not possible (Arc); just flush
        if let Ok(mut f) = self.inner.lock() {
            let _ = f.flush();
        }
        Ok(())
    }
}
