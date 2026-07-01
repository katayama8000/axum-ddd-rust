#![cfg(test)]
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::mysql::Mysql;

/// Starts a fresh, isolated MySQL container per call, so tests using this no longer need to
/// coordinate with each other (no shared tables, no `#[serial]`/`#[file_serial]` needed).
/// The returned container must be kept alive for as long as the pool is used — dropping it
/// stops the container and breaks the connection.
pub async fn setup() -> (ContainerAsync<Mysql>, MySqlPool) {
    let container = Mysql::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(3306).await.unwrap();
    let database_url = format!("mysql://root@{host}:{port}/test");

    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS circles (
            id CHAR(36) NOT NULL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            capacity INT NOT NULL,
            owner_id CHAR(36) NOT NULL
        );",
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
        );",
    )
    .execute(&pool)
    .await
    .unwrap();

    (container, pool)
}
