use anyhow::{Error, Result};
use async_trait::async_trait;
use domain::{
    aggregate::circle::Circle,
    interface::circle_duplicate_checker_interface::CircleDuplicateCheckerInterface,
};
use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct CircleDuplicateCheckerWithMySql {
    db: MySqlPool,
}

impl CircleDuplicateCheckerWithMySql {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CircleDuplicateCheckerInterface for CircleDuplicateCheckerWithMySql {
    async fn check_circle_duplicate(&self, circle: &Circle) -> Result<(), Error> {
        // SQL query to check if a circle with the same name already exists
        let query = "SELECT * FROM circles WHERE name = ?";
        let record = sqlx::query(query)
            .bind(circle.name())
            .fetch_optional(&self.db)
            .await?;

        // If a record is found, it indicates a duplicate circle name
        if record.is_some() {
            return Err(anyhow::anyhow!("Circle name already exists"));
        }

        Ok(())
    }
}
