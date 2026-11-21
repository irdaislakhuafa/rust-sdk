#[cfg(test)]
mod tests {
    use crate::storage::{
        init, Config, DelParams, PutParams, StorageType, UrlParams,
    };

    fn dummy_config() -> Config {
        Config {
            storage_type: StorageType::Dummy,
            base_dir: String::new(),
            host: String::new(),
            port: String::new(),
            ssl: false,
            access_key_id: String::new(),
            access_key_secret: String::new(),
            token: String::new(),
            region: String::new(),
            max_retries: 0,
        }
    }

    #[test]
    fn test_dummy_put() {
        struct Case {
            name: &'static str,
            dir_name: &'static str,
            file_name: &'static str,
            file_size: i64,
            expected_dir: &'static str,
        }

        let cases = [
            Case {
                name: "basic dir",
                dir_name: "dir",
                file_name: "file.txt",
                file_size: 5,
                expected_dir: "dir/",
            },
            Case {
                name: "already normalized dir",
                dir_name: "dir/",
                file_name: "file.txt",
                file_size: 5,
                expected_dir: "dir/",
            },
            Case {
                name: "nested dir",
                dir_name: "a/b",
                file_name: "data.json",
                file_size: 42,
                expected_dir: "a/b/",
            },
            Case {
                name: "empty dir",
                dir_name: "",
                file_name: "root.txt",
                file_size: 0,
                expected_dir: "/",
            },
        ];

        let storage = init(dummy_config()).unwrap();

        for case in cases {
            let put = storage
                .put(
                    (),
                    PutParams {
                        dir_name: case.dir_name.to_string(),
                        file_name: case.file_name.to_string(),
                        file_size: case.file_size,
                        ..Default::default()
                    },
                )
                .unwrap_or_else(|e| panic!("put failed for {}: {}", case.name, e));

            assert_eq!(put.dir_name, case.expected_dir, "case: {}", case.name);
            assert_eq!(put.file_name, case.file_name, "case: {}", case.name);
            assert_eq!(
                put.full_path,
                format!("{}{}", case.expected_dir, case.file_name),
                "case: {}",
                case.name
            );
            assert_eq!(put.region, "dummy", "case: {}", case.name);
            assert_eq!(put.tag, "dummy", "case: {}", case.name);
            assert_eq!(put.size, case.file_size, "case: {}", case.name);
        }
    }

    #[test]
    fn test_dummy_del() {
        struct Case {
            name: &'static str,
            full_path: &'static str,
            force: bool,
        }

        let cases = [
            Case {
                name: "default params",
                full_path: "",
                force: false,
            },
            Case {
                name: "with full path",
                full_path: "dir/file.txt",
                force: false,
            },
            Case {
                name: "force delete",
                full_path: "dir/file.txt",
                force: true,
            },
        ];

        let storage = init(dummy_config()).unwrap();

        for case in cases {
            let result = storage.del(
                (),
                DelParams {
                    full_path: case.full_path.to_string(),
                    force: case.force,
                },
            );
            assert!(
                result.is_ok(),
                "del should succeed for case: {}",
                case.name
            );
        }
    }

    #[test]
    fn test_dummy_url() {
        struct Case {
            name: &'static str,
            full_path: &'static str,
        }

        let cases = [
            Case {
                name: "nested path",
                full_path: "a/b",
            },
            Case {
                name: "single segment",
                full_path: "file.txt",
            },
            Case {
                name: "empty path",
                full_path: "",
            },
        ];

        let storage = init(dummy_config()).unwrap();

        for case in cases {
            let url = storage
                .url(
                    (),
                    UrlParams {
                        full_path: case.full_path.to_string(),
                        ..Default::default()
                    },
                )
                .unwrap_or_else(|e| panic!("url failed for {}: {}", case.name, e));

            assert_eq!(url.scheme, "https", "case: {}", case.name);
            assert_eq!(url.host, "dummy.com", "case: {}", case.name);
            assert_eq!(url.full_path, case.full_path, "case: {}", case.name);
            assert_eq!(
                url.full_url,
                format!("dummy.com/{}", case.full_path),
                "case: {}",
                case.name
            );
            assert!(url.raw_query.is_empty(), "case: {}", case.name);
        }
    }
}