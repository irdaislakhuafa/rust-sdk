#[cfg(test)]
mod tests {
    use crate::storage::helper::{format_bytes, normalize_dir, percent_bytes};

    #[test]
    fn test_normalize_dir() {
        assert_eq!(normalize_dir(""), "/");
        assert_eq!(normalize_dir("dir"), "dir/");
        assert_eq!(normalize_dir("dir/"), "dir/");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0.00 B");
        assert_eq!(format_bytes(1023), "1023.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024), "1.00 TB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024 * 1024), "1.00 PB");
    }

    #[test]
    fn test_percent_bytes() {
        assert_eq!(percent_bytes(0, 0), 100.0);
        assert_eq!(percent_bytes(0, 1024 * 1024 * 10), 0.0);
        assert_eq!(percent_bytes(1024 * 1024, 1024 * 1024 * 10), 10.0);
        assert_eq!(percent_bytes(1024 * 1024 * 5, 1024 * 1024 * 10), 50.0);
    }
}
