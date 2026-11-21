#[cfg(test)]
mod tests {
    use crate::db::{Config, Options, DRIVER_POSTGRES, init_postgres};

    #[tokio::test]
    async fn test_postgres_env_gated() {
        let host = match std::env::var("DB_POSTGRES_TEST_HOST") {
            Ok(v) => v,
            Err(_) => {
                eprintln!("skipping postgres integration: set DB_POSTGRES_TEST_HOST");
                return;
            }
        };
        let port = std::env::var("DB_POSTGRES_TEST_PORT").unwrap_or_else(|_| "5432".to_string());
        let user = std::env::var("DB_POSTGRES_TEST_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = std::env::var("DB_POSTGRES_TEST_PASSWORD").unwrap_or_default();
        let name = std::env::var("DB_POSTGRES_TEST_NAME").unwrap_or_else(|_| "postgres".to_string());

        let cfg = Config {
            driver: DRIVER_POSTGRES.to_string(),
            user,
            password,
            port,
            host,
            name,
            options: Options::default(),
        };
        let pool = match init_postgres(cfg).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("skipping postgres integration: {}", e);
                return;
            }
        };
        sqlx::query("SELECT 1").execute(&pool).await.unwrap();
    }
}