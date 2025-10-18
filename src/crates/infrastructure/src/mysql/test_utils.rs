#![cfg(test)]
use dotenv::from_filename;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

pub async fn setup() -> MySqlPool {
    let db_type = env::var("DB_TYPE").unwrap_or_else(|_| "mysql".to_string());
    let env_file = format!(".env.{}", db_type);
    from_filename(env_file).ok();

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        env::var("MYSQL_USER").unwrap(),
        env::var("MYSQL_PASSWORD").unwrap(),
        env::var("MYSQL_HOST").unwrap(),
        env::var("MYSQL_PORT").unwrap(),
        env::var("MYSQL_NAME").unwrap()
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap();

    // Create tables if they don't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS circles (
            id CHAR(36) NOT NULL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            capacity INT NOT NULL,
            owner_id CHAR(36) NOT NULL
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS members (
            id CHAR(36) NOT NULL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            grade INT NOT NULL,
            circle_id CHAR(36),
            age INT NOT NULL DEFAULT 20,
            major VARCHAR(255) NOT NULL DEFAULT 'other',
            FOREIGN KEY (circle_id) REFERENCES circles(id) ON DELETE CASCADE
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

pub async fn clean_up(pool: MySqlPool) {
    sqlx::query("DELETE FROM members").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM circles").execute(&pool).await.unwrap();
    pool.close().await;
}