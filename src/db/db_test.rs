#[cfg(test)]
mod tests {
    use crate::db::{Config, Options, DRIVER_MYSQL, DRIVER_POSTGRES, DRIVER_SQLITE, init};
    use crate::codes::Code;
    use crate::errors;

    fn is_not_implemented(err: &crate::errors::SdkError) -> bool {
        errors::get_code(err.as_ref()) == Code::NotImplemented
    }

    fn is_sql_init(err: &crate::errors::SdkError) -> bool {
        errors::get_code(err.as_ref()) == Code::SqlInit
    }

    #[tokio::test]
    async fn test_init_unknown_driver() {
        let cfg = Config {
            driver: "unknown".to_string(),
            user: String::new(),
            password: String::new(),
            port: String::new(),
            host: String::new(),
            name: String::new(),
            options: Options::default(),
        };
        let err = init(cfg).await.unwrap_err();
        assert!(is_not_implemented(&err));
    }

    #[test]
    fn test_build_url_postgres() {
        let cfg = Config {
            driver: DRIVER_POSTGRES.to_string(),
            user: "u".to_string(),
            password: "p".to_string(),
            port: "5432".to_string(),
            host: "h".to_string(),
            name: "db".to_string(),
            options: Options {
                ssl_mode: "disable".to_string(),
                ..Default::default()
            },
        };
        let url = crate::db::postgres::build_url(&cfg);
        assert_eq!(url, "postgres://u:p@h:5432/db?sslmode=disable");
    }

    #[test]
    fn test_build_url_mysql() {
        let cfg = Config {
            driver: DRIVER_MYSQL.to_string(),
            user: "u".to_string(),
            password: "p".to_string(),
            port: "3306".to_string(),
            host: "h".to_string(),
            name: "db".to_string(),
            options: Options::default(),
        };
        let url = crate::db::mysql::build_url(&cfg);
        assert_eq!(url, "mysql://u:p@h:3306/db");
    }

    #[tokio::test]
    async fn test_init_sqlite_memory() {
        let cfg = Config {
            driver: DRIVER_SQLITE.to_string(),
            user: String::new(),
            password: String::new(),
            port: String::new(),
            host: String::new(),
            name: "test".to_string(),
            options: Options {
                in_memory: true,
                ..Default::default()
            },
        };
        // in-memory uses sqlite::memory: URL, name ignored
        let pool = init(cfg).await.unwrap();
        // ping
        sqlx::query("SELECT 1").execute(&pool).await.unwrap();
    }
}