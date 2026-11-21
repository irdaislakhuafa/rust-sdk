#[cfg(test)]
mod tests {
    use crate::db::{Config, Options, DRIVER_SQLITE, init_sqlite};

    #[tokio::test]
    async fn test_sqlite_file_roundtrip() {
        let path = "/tmp/rust_sdk_db_test";
        let _ = std::fs::remove_file(format!("{}.db", path));
        let cfg = Config {
            driver: DRIVER_SQLITE.to_string(),
            user: String::new(),
            password: String::new(),
            port: String::new(),
            host: String::new(),
            name: path.to_string(),
            options: Options::default(),
        };
        let pool = init_sqlite(cfg).await.unwrap();
        sqlx::query("CREATE TABLE t (x INTEGER)").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO t (x) VALUES (42)").execute(&pool).await.unwrap();
        let row: (i64,) = sqlx::query_as("SELECT x FROM t").fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 42);
        let _ = std::fs::remove_file(format!("{}.db", path));
    }

    #[tokio::test]
    async fn test_sqlite_memory_select_one() {
        let cfg = Config {
            driver: DRIVER_SQLITE.to_string(),
            user: String::new(),
            password: String::new(),
            port: String::new(),
            host: String::new(),
            name: "rt".to_string(),
            options: Options {
                in_memory: true,
                ..Default::default()
            },
        };
        let pool = init_sqlite(cfg).await.unwrap();
        let row: (i64,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 1);
    }
}