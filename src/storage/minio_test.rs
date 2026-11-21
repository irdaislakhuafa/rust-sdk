#[cfg(test)]
mod tests {
    use crate::storage::{init, Config, DelParams, PutParams, StorageType, UrlParams};
    use s3::Bucket;
    use s3::creds::Credentials;
    use s3::region::Region;
    use std::env;
    use std::io::Cursor;
    use url::Url;

    fn env_config() -> Option<Config> {
        let access_key = env::var("MINIO_TEST_ACCESS_KEY").ok()?;
        let secret_key = env::var("MINIO_TEST_SECRET_KEY").ok()?;

        let host = env::var("MINIO_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("MINIO_TEST_PORT").unwrap_or_else(|_| "9000".to_string());
        let bucket = env::var("MINIO_TEST_BUCKET").unwrap_or_else(|_| "rust-sdk-test".to_string());
        let region = env::var("MINIO_TEST_REGION").unwrap_or_else(|_| "us-east-1".to_string());
        let ssl = env::var("MINIO_TEST_SSL")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let token = env::var("MINIO_TEST_TOKEN").unwrap_or_default();

        Some(Config {
            storage_type: StorageType::Minio,
            base_dir: bucket,
            host,
            port,
            ssl,
            access_key_id: access_key,
            access_key_secret: secret_key,
            token,
            region,
            max_retries: 0,
        })
    }

    fn test_bucket(cfg: &Config) -> Box<Bucket> {
        let scheme = if cfg.ssl { "https" } else { "http" };
        let host = cfg.host.trim();
        let port = cfg.port.trim();
        let endpoint = if port.is_empty() {
            format!("{}://{}", scheme, host)
        } else if port.starts_with(':') {
            format!("{}://{}{}", scheme, host, port)
        } else {
            format!("{}://{}:{}", scheme, host, port)
        };

        let credentials = Credentials::new(
            Some(cfg.access_key_id.as_str()),
            Some(cfg.access_key_secret.as_str()),
            if cfg.token.is_empty() {
                None
            } else {
                Some(cfg.token.as_str())
            },
            None,
            None,
        )
        .expect("valid test credentials");

        let region = Region::Custom {
            region: cfg.region.clone(),
            endpoint,
        };

        Bucket::new(&cfg.base_dir, region, credentials)
            .expect("create test bucket client")
            .with_path_style()
    }

    #[test]
    fn test_minio_init_invalid_endpoints() {
        let cases = [
            ("unreachable port", "127.0.0.1", "1"),
            ("empty host", "", "9000"),
        ];

        for (name, host, port) in cases {
            let mut cfg = Config {
                storage_type: StorageType::Minio,
                base_dir: "test-bucket".to_string(),
                host: host.to_string(),
                port: port.to_string(),
                ssl: false,
                access_key_id: "minioadmin".to_string(),
                access_key_secret: "minioadmin".to_string(),
                token: String::new(),
                region: "us-east-1".to_string(),
                max_retries: 0,
            };
            cfg.parse_default();
            assert!(
                init(cfg).is_err(),
                "init should fail for invalid endpoint case: {}",
                name
            );
        }
    }

    #[test]
    fn test_minio_crud_table_driven() {
        let Some(cfg) = env_config() else {
            eprintln!("skipping MinIO CRUD test: set MINIO_TEST_ACCESS_KEY and MINIO_TEST_SECRET_KEY");
            return;
        };

        let storage = init(cfg.clone()).expect("init MinIO storage");
        let bucket = test_bucket(&cfg);

        struct Case {
            name: &'static str,
            content_type: &'static str,
            body: &'static [u8],
        }

        let cases = [
            Case {
                name: "text file",
                content_type: "text/plain",
                body: b"hello minio",
            },
            Case {
                name: "json file",
                content_type: "application/json",
                body: b"{\"ok\":true}",
            },
            Case {
                name: "empty file",
                content_type: "application/octet-stream",
                body: b"",
            },
        ];

        for case in cases {
            let dir_name = format!("tests/{}/", uuid::Uuid::new_v4());
            let file_name = "payload.bin";

            let put = storage
                .put(
                    (),
                    PutParams {
                        dir_name: dir_name.clone(),
                        file_name: file_name.to_string(),
                        file_buffer: Some(Box::new(Cursor::new(case.body.to_vec()))),
                        file_size: case.body.len() as i64,
                        content_type: case.content_type.to_string(),
                        ..Default::default()
                    },
                )
                .unwrap_or_else(|e| panic!("put failed for {}: {}", case.name, e));

            assert_eq!(put.dir_name, dir_name, "case: {}", case.name);
            assert_eq!(put.file_name, file_name, "case: {}", case.name);
            assert_eq!(put.full_path, format!("{}{}", dir_name, file_name), "case: {}", case.name);
            assert_eq!(put.region, cfg.region, "case: {}", case.name);
            assert_eq!(put.size, case.body.len() as i64, "case: {}", case.name);
            assert!(!put.tag.is_empty(), "case: {}", case.name);

            let head = bucket.head_object(&put.full_path);
            assert!(head.is_ok(), "object should exist after put: {}", case.name);

            let url_result = storage
                .url(
                    (),
                    UrlParams {
                        full_path: put.full_path.clone(),
                        expire_duration: std::time::Duration::from_secs(3600),
                        ..Default::default()
                    },
                )
                .unwrap_or_else(|e| panic!("url failed for {}: {}", case.name, e));

            assert!(!url_result.scheme.is_empty(), "case: {}", case.name);
            assert!(!url_result.host.is_empty(), "case: {}", case.name);
            assert!(!url_result.full_url.is_empty(), "case: {}", case.name);
            assert!(
                Url::parse(&url_result.full_url).is_ok(),
                "presigned url should parse: {}",
                case.name
            );

            storage
                .del(
                    (),
                    DelParams {
                        full_path: put.full_path.clone(),
                        force: false,
                    },
                )
                .unwrap_or_else(|e| panic!("delete failed for {}: {}", case.name, e));

            let head_after = bucket.head_object(&put.full_path);
            assert!(
                head_after.is_err(),
                "object should not exist after delete: {}",
                case.name
            );
        }
    }
}