#[cfg(test)]
mod tests {
    use crate::caches::{init, Config};
    use crate::codes::Code;
    use crate::errors;

    fn is_cache_key_not_found(err: &crate::errors::SdkError) -> bool {
        errors::get_code(err.as_ref()) == Code::CacheKeyNotFound
    }

    #[test]
    fn test_sqlite_cache() {
        let dir = "/tmp/".to_string();
        let _ = std::fs::remove_file(format!("{}caches", dir));

        let cache = init::<String>(Config::sqlite(dir.clone())).unwrap();
        let cb_calls = std::sync::atomic::AtomicUsize::new(0);
        let value = cache
            .remember("key_3s", 3, &mut || {
                cb_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok("value_3s".to_string())
            })
            .unwrap();
        assert_eq!(value, "value_3s");
        let value2 = cache
            .remember("key_3s", 3, &mut || {
                cb_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok("value_3s".to_string())
            })
            .unwrap();
        assert_eq!(value2, "value_3s");
        assert_eq!(cb_calls.load(std::sync::atomic::Ordering::SeqCst), 1);

        let v = cache.forget("key_3s").unwrap();
        assert_eq!(v, "value_3s");
        let err = cache.forget("key_3s").unwrap_err();
        assert!(is_cache_key_not_found(&err));

        cache
            .remember("key_3s", 3, &mut || Ok("value_3s".to_string()))
            .unwrap();
        let v = cache.get("key_3s").unwrap();
        assert_eq!(v, "value_3s");

        let cache2 = init::<String>(Config::sqlite(dir.clone())).unwrap();
        cache2.clear();
        cache2
            .remember("key_3s_fn", 3, &mut || Ok("value_3s_fn".to_string()))
            .unwrap();
        let res = cache2
            .forget_fn(&mut |key| {
                if key.contains("key_3s_fn") {
                    cache2.forget(key)
                } else {
                    cache2.get(key)
                }
            })
            .unwrap();
        assert_eq!(res, Some("value_3s_fn".to_string()));
        let err = cache2.forget("key_3s_fn").unwrap_err();
        assert!(is_cache_key_not_found(&err));

        let _ = std::fs::remove_file(format!("{}caches", dir));
    }
}