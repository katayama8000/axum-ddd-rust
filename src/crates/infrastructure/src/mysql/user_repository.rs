use anyhow::Context;
use domain::{
    aggregate::{
        user::User,
        value_object::{email::Email, user_id::UserId},
    },
    interface::user_repository_interface::UserRepositoryInterface,
};
use sqlx::Row;

use crate::db_schema::user_data::UserData;

#[derive(Clone, Debug)]
pub struct UserRepository {
    db: sqlx::MySqlPool,
}

impl UserRepository {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }

    async fn find_one(
        &self,
        query: sqlx::query::Query<'_, sqlx::MySql, sqlx::mysql::MySqlArguments>,
    ) -> Result<Option<User>, anyhow::Error> {
        let row = query
            .fetch_optional(&self.db)
            .await
            .context("Failed to fetch user")?;

        match row {
            None => Ok(None),
            Some(row) => {
                let user_data = UserData {
                    id: row.get::<String, _>("id"),
                    email: row.get::<String, _>("email"),
                    password_hash: row.get::<String, _>("password_hash"),
                };
                User::try_from(user_data).map(Some)
            }
        }
    }
}

#[async_trait::async_trait]
impl UserRepositoryInterface for UserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, anyhow::Error> {
        tracing::info!("find_user_by_email");
        let query = sqlx::query("SELECT id, email, password_hash FROM users WHERE email = ?")
            .bind(email.to_string());

        self.find_one(query).await
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, anyhow::Error> {
        tracing::info!("find_user_by_id");
        let query = sqlx::query("SELECT id, email, password_hash FROM users WHERE id = ?")
            .bind(user_id.to_string());

        self.find_one(query).await
    }

    async fn create(&self, user: &User) -> Result<(), anyhow::Error> {
        tracing::info!("create_user");
        let user_data = UserData::from(user.clone());

        sqlx::query("INSERT INTO users (id, email, password_hash) VALUES (?, ?, ?)")
            .bind(user_data.id)
            .bind(user_data.email)
            .bind(user_data.password_hash)
            .execute(&self.db)
            .await
            .context("Failed to insert user")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::aggregate::value_object::password_hash::PasswordHash;

    use crate::mysql::test_utils::setup;

    fn user(email: &str) -> anyhow::Result<User> {
        Ok(User::new(
            Email::new(email)?,
            PasswordHash::new("$argon2id$v=19$m=19456,t=2,p=1$abc$def")?,
        ))
    }

    #[tokio::test]
    async fn test_create_and_find_by_email() -> anyhow::Result<()> {
        let (_container, pool) = setup().await;
        let repository = UserRepository::new(pool);
        let user = user("user@example.com")?;

        repository.create(&user).await?;
        let found = repository
            .find_by_email(&Email::new("user@example.com")?)
            .await?;

        assert_eq!(found, Some(user));
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_email_is_case_insensitive() -> anyhow::Result<()> {
        let (_container, pool) = setup().await;
        let repository = UserRepository::new(pool);
        let user = user("User@Example.COM")?;

        repository.create(&user).await?;
        let found = repository
            .find_by_email(&Email::new("user@example.com")?)
            .await?;

        assert_eq!(found, Some(user));
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id() -> anyhow::Result<()> {
        let (_container, pool) = setup().await;
        let repository = UserRepository::new(pool);
        let user = user("user@example.com")?;

        repository.create(&user).await?;
        let found = repository.find_by_id(&user.id).await?;

        assert_eq!(found, Some(user));
        Ok(())
    }

    #[tokio::test]
    async fn test_find_returns_none_when_absent() -> anyhow::Result<()> {
        let (_container, pool) = setup().await;
        let repository = UserRepository::new(pool);

        let found = repository
            .find_by_email(&Email::new("nobody@example.com")?)
            .await?;

        assert_eq!(found, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_duplicate_email_is_rejected_by_the_database() -> anyhow::Result<()> {
        let (_container, pool) = setup().await;
        let repository = UserRepository::new(pool);

        repository.create(&user("user@example.com")?).await?;
        // The unique index, not the use case, is what ultimately enforces uniqueness.
        let result = repository.create(&user("user@example.com")?).await;

        assert!(result.is_err());
        Ok(())
    }
}
