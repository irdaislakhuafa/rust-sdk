// file sink with rotation policies (size-based and date-based)

use crate::codes::Code;
use crate::errors::{new_with_code, SdkError};
use crate::log::sink::LogSink;
use super::storage::DateRotationInterval;
use chrono::{DateTime, Utc};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub trait RotatePolicy: Send + Sync + std::fmt::Debug {
    fn should_rotate(&self, path: &Path) -> bool;
    fn rotate(&self, path: &Path) -> Result<(), SdkError>;
}

// ---------- Size-based policy (kept for OCP / users who want caps) ----------

#[derive(Debug)]
pub struct SizeRotatePolicy {
    pub max_size_bytes: u64,
    pub max_backups: usize,
}

impl RotatePolicy for SizeRotatePolicy {
    fn should_rotate(&self, path: &Path) -> bool {
        if self.max_size_bytes == 0 {
            return false;
        }
        let current_size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        current_size >= self.max_size_bytes
    }
    fn rotate(&self, path: &Path) -> Result<(), SdkError> {
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
                new_with_code(Code::LogWriteFailure, format!("rotate rename failed: {}", e))
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

// ---------- Date-based policy (default) ----------

pub struct DateRotatePolicy {
    pub interval: DateRotationInterval,
    pub max_backups: usize,
    pub max_age_days: i64,
    pub date_format: String,
    // Test seam: override "now". None => real clock.
    pub now_fn: Option<Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>>,
}

impl std::fmt::Debug for DateRotatePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DateRotatePolicy")
            .field("interval", &self.interval)
            .field("max_backups", &self.max_backups)
            .field("max_age_days", &self.max_age_days)
            .field("date_format", &self.date_format)
            .field("now_fn", &self.now_fn.is_some())
            .finish()
    }
}

impl DateRotatePolicy {
    fn now(&self) -> DateTime<Utc> {
        match &self.now_fn {
            Some(f) => f(),
            None => Utc::now(),
        }
    }

    fn period_key(dt: DateTime<Utc>, interval: DateRotationInterval) -> String {
        match interval {
            DateRotationInterval::Hourly => dt.format("%Y-%m-%d-%H").to_string(),
            DateRotationInterval::Daily => dt.format("%Y-%m-%d").to_string(),
        }
    }

    fn default_format(interval: DateRotationInterval) -> &'static str {
        match interval {
            DateRotationInterval::Hourly => "%Y-%m-%d-%H",
            DateRotationInterval::Daily => "%Y-%m-%d",
        }
    }
}

impl RotatePolicy for DateRotatePolicy {
    fn should_rotate(&self, path: &Path) -> bool {
        if !path.exists() {
            return false;
        }
        let mtime = fs::metadata(path).ok().and_then(|m| m.modified().ok());
        let file_dt = match mtime {
            Some(t) => DateTime::<Utc>::from(t),
            None => return false,
        };
        let now = self.now();
        let file_period = Self::period_key(file_dt, self.interval);
        let now_period = Self::period_key(now, self.interval);
        file_period != now_period
    }

    fn rotate(&self, path: &Path) -> Result<(), SdkError> {
        let fmt = if self.date_format.is_empty() {
            Self::default_format(self.interval).to_string()
        } else {
            self.date_format.clone()
        };

        // Determine backup date from file mtime (the period being closed).
        let file_dt = fs::metadata(path)
            .ok()
            .and_then(|m| m.modified().ok())
            .map(DateTime::<Utc>::from)
            .unwrap_or_else(|| self.now());
        let date_str = file_dt.format(&fmt).to_string();

        // Build backup path: {path}.{date}, with counter on collision.
        let mut backup = PathBuf::from(format!("{}.{}", path.display(), date_str));
        if backup.exists() {
            let mut i = 1;
            loop {
                let candidate = PathBuf::from(format!("{}.{}.{}", path.display(), date_str, i));
                if !candidate.exists() {
                    backup = candidate;
                    break;
                }
                i += 1;
            }
        }

        if path.exists() {
            fs::rename(path, &backup).map_err(|e| {
                new_with_code(Code::LogWriteFailure, format!("rotate rename failed: {}", e))
            })?;
        }

        // Cleanup old backups.
        self.cleanup_backups(path);
        Ok(())
    }
}

impl DateRotatePolicy {
    fn cleanup_backups(&self, path: &Path) {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let base_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let prefix = format!("{}.", base_name);

        let mut backups: Vec<PathBuf> = fs::read_dir(parent)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().starts_with(&prefix))
                    .unwrap_or(false)
            })
            .collect();

        backups.sort();

        // max_backups: keep last N
        if self.max_backups > 0 && backups.len() > self.max_backups {
            let to_remove = backups.len() - self.max_backups;
            for p in &backups[..to_remove] {
                let _ = fs::remove_file(p);
            }
            backups = backups[to_remove..].to_vec();
        }

        // max_age_days: remove backups older than threshold (by mtime)
        if self.max_age_days > 0 {
            let cutoff = self.now() - chrono::Duration::days(self.max_age_days);
            for p in &backups {
                if let Ok(meta) = fs::metadata(p) {
                    if let Ok(mtime) = meta.modified() {
                        let dt = DateTime::<Utc>::from(mtime);
                        if dt < cutoff {
                            let _ = fs::remove_file(p);
                        }
                    }
                }
            }
        }
    }
}

// ---------- FileSink ----------

#[derive(Debug)]
pub struct FileSink {
    path: PathBuf,
    policy: Option<Arc<dyn RotatePolicy>>,
    inner: Arc<Mutex<std::fs::File>>,
}

impl FileSink {
    pub fn new(path: impl AsRef<Path>, policy: Option<Arc<dyn RotatePolicy>>) -> Result<Self, SdkError> {
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
            .map_err(|e| {
                new_with_code(Code::LogInitFailure, format!("open file failed: {}", e))
            })?;
        Ok(Self {
            path,
            policy,
            inner: Arc::new(Mutex::new(file)),
        })
    }
}

impl LogSink for FileSink {
    fn write(&self, entry: &[u8]) -> Result<(), SdkError> {
        if let Some(policy) = &self.policy {
            if policy.should_rotate(&self.path) {
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
                let mut f = self.inner.lock().map_err(|_| {
                    new_with_code(Code::LogWriteFailure, "lock failed".to_string())
                })?;
                *f = new_file;
            }
        }
        let mut f = self.inner.lock().map_err(|_| {
            new_with_code(Code::LogWriteFailure, "lock failed".to_string())
        })?;
        f.write_all(entry).map_err(|e| {
            new_with_code(Code::LogWriteFailure, format!("write failed: {}", e))
        })?;
        let _ = f.flush();
        Ok(())
    }
    fn cleanup(&self) -> Result<(), SdkError> {
        if let Ok(mut f) = self.inner.lock() {
            let _ = f.flush();
        }
        Ok(())
    }
}
