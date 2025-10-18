use dotenv::from_filename;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[derive(Debug, Clone)]
pub(crate) struct DbConfig {
    db_user: String,
    db_password: String,
    db_host: String,
    db_name: String,
}

impl DbConfig {
    fn from_env() -> Self {
        let db_type = env::var("DB_TYPE").unwrap_or_else(|_| "mysql".to_string());
        let env_file = format!(".env.{}", db_type);
        from_filename(env_file).ok();

        Self {
            db_user: env::var("MYSQL_USER").expect("MYSQL_USER must be set"),
            db_password: env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set"),
            db_host: env::var("MYSQL_HOST").expect("MYSQL_HOST must be set"),
            db_name: env::var("MYSQL_NAME").expect("MYSQL_NAME must be set"),
        }
    }

    fn connection(&self) -> String {
        println!(
            "mysql://{}:{}@{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_name
        );
        format!(
            "mysql://{}:{}@{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_name
        )
    }
}

pub async fn connect() -> Result<sqlx::MySqlPool, sqlx::Error> {
    let config = DbConfig::from_env();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection())
        .await?;
    Ok(pool)
}

#[cfg(test)]
pub async fn connect_test() -> Result<sqlx::MySqlPool, sqlx::Error> {
    // TODO: build a db connection for testing
    let config = DbConfig::from_env();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection())
        .await?;
    Ok(pool)
}
