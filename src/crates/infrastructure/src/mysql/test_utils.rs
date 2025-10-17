#![cfg(test)]
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;
use domain::aggregate::value_object::circle_id::CircleId;

pub async fn setup() -> MySqlPool {
    dotenv().ok();
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        env::var("MYSQL_USER").unwrap(),
        env::var("MYSQL_PASSWORD").unwrap(),
        env::var("MYSQL_HOST").unwrap(),
        env::var("MYSQL_PORT").unwrap(),
        env::var("MYSQL_NAME").unwrap()
    );
    MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap()
}

pub async fn clean_up(pool: MySqlPool, circle_id: &CircleId) {
    sqlx::query("DELETE FROM circles WHERE id = ?")
        .bind(circle_id.to_string())
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;
}