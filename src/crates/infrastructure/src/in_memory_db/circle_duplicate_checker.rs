use anyhow::{Error, Result};
use async_trait::async_trait;
use domain::{
    aggregate::circle::Circle,
    interface::circle_duplicate_checker_interface::CircleDuplicateCheckerInterface,
};

use crate::{db_schema::circle_data::CircleData, in_memory_db::db::Db};

#[derive(Clone, Debug)]
pub struct CircleDuplicateChecker {
    db: Db,
}

impl CircleDuplicateChecker {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

#[async_trait]
impl CircleDuplicateCheckerInterface for CircleDuplicateChecker {
    async fn check_circle_duplicate(&self, circle: &Circle) -> Result<(), Error> {
        let id = circle.id();
        match self.db.get::<CircleData, _>(&id.to_string())? {
            Some(_) => Err(Error::msg("Circle already exists")),
            None => Ok(()),
        }
    }
}
