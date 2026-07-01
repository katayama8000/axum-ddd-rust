use anyhow::{Error, Result};
use async_trait::async_trait;
use domain::{
    aggregate::circle::Circle,
    interface::circle_duplicate_checker_interface::CircleDuplicateCheckerInterface,
};
use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct CircleDuplicateChecker {
    db: MySqlPool,
}

impl CircleDuplicateChecker {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CircleDuplicateCheckerInterface for CircleDuplicateChecker {
    async fn check_circle_duplicate(&self, circle: &Circle) -> Result<(), Error> {
        let query = "SELECT * FROM circles WHERE name = ?";
        let record = sqlx::query(query)
            .bind(circle.name())
            .fetch_optional(&self.db)
            .await?;

        if record.is_some() {
            return Err(anyhow::anyhow!("Circle name already exists"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::mysql::test_utils::{clean_up, setup};
    use serial_test::serial;

    use super::*;
    use domain::aggregate::{
        circle::Circle,
        member::Member,
        value_object::{circle_id::CircleId, grade::Grade, major::Major},
    };

    fn create_test_circle(name: &str) -> Circle {
        let owner_grade = Grade::Third;
        let owner_major = Major::try_from("Computer Science").unwrap();
        let owner = Member::new("owner".to_string(), 21, owner_grade, owner_major);
        Circle::create(name.to_string(), owner, 10).unwrap()
    }

    #[tokio::test]
    #[serial]
    async fn check_circle_duplicate_exists() {
        let pool = setup().await;
        let checker = CircleDuplicateChecker::new(pool.clone());

        // Use a unique name for this test to avoid conflicts with parallel tests
        let unique_name = format!("Test Circle {}", CircleId::gen());
        let circle = create_test_circle(&unique_name);

        // Create a circle with the same name
        sqlx::query("INSERT INTO circles (id, name, owner_id, capacity) VALUES (?, ?, ?, ?)")
            .bind(circle.id().to_string())
            .bind(circle.name())
            .bind(circle.owner.id.to_string())
            .bind(circle.capacity)
            .execute(&pool)
            .await
            .unwrap();

        let result = checker.check_circle_duplicate(&circle).await;
        assert!(result.is_err());
        clean_up(pool).await;
    }

    #[tokio::test]
    #[serial]
    async fn check_circle_duplicate_not_exists() {
        let pool = setup().await;
        let checker = CircleDuplicateChecker::new(pool.clone());

        // Use a unique name for this test to avoid conflicts with parallel tests
        let unique_name = format!("New Circle {}", CircleId::gen());
        let circle = create_test_circle(&unique_name);

        let result = checker.check_circle_duplicate(&circle).await;
        assert!(result.is_ok());
        clean_up(pool).await;
    }
}
