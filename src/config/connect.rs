use sqlx::{mysql::MySqlPoolOptions, Pool};

pub async fn connect() -> Result<Pool<sqlx::MySql>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://myuser:mypassword@mariadb/mydatabase")
        .await?;
    Ok(pool)
}
