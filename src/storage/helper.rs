// storage helpers (port of go-sdk/storage/helper.go)

pub fn normalize_dir(dir: &str) -> String {
    if dir.ends_with('/') {
        dir.to_string()
    } else if dir.is_empty() {
        "/".to_string()
    } else {
        format!("{}/", dir)
    }
}

pub fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes < 1024 {
        return format!("{:.2} B", bytes as f64);
    }
    let mut value = bytes as f64;
    let mut idx = 0usize;
    while value >= 1024.0 && idx < units.len() - 1 {
        value /= 1024.0;
        idx += 1;
    }
    format!("{:.2} {}", value, units[idx])
}

pub fn percent_bytes(used: u64, total: u64) -> f64 {
    if total <= 0 {
        return 100.0;
    }
    (used as f64 / total as f64) * 100.0
}


