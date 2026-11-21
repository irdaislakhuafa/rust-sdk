#[cfg(test)]
mod tests {
    use crate::db::{Config, Options, DRIVER_MYSQL, init_mysql};

    #[tokio::test]
    async fn test_mysql_env_gated() {
        let host = match std::env::var("DB_MYSQL_TEST_HOST") {
            Ok(v) => v,
            Err(_) => {
                eprintln!("skipping mysql integration: set DB_MYSQL_TEST_HOST");
                return;
            }
        };
        let port = std::env::var("DB_MYSQL_TEST_PORT").unwrap_or_else(|_| "3306".to_string());
        let user = std::env::var("DB_MYSQL_TEST_USER").unwrap_or_else(|_| "root".to_string());
        let password = std::env::var("DB_MYSQL_TEST_PASSWORD").unwrap_or_default();
        let name = std::env::var("DB_MYSQL_TEST_NAME").unwrap_or_else(|_| "test".to_string());

        let cfg = Config {
            driver: DRIVER_MYSQL.to_string(),
            user,
            password,
            port,
            host,
            name,
            options: Options::default(),
        };
        let pool = match init_mysql(cfg).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("skipping mysql integration: {}", e);
                return;
            }
        };
        sqlx::query("SELECT 1").execute(&pool).await.unwrap();
    }
}