use anyhow::Error;

use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    repository::circle_repository_trait::CircleRepositoryTrait,
};

use super::db::Db;

pub struct CircleRepository {
    db: Db,
}

impl CircleRepository {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

impl CircleRepositoryTrait for CircleRepository {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error> {
        match self.db.find(&circle_id.to_string()) {
            Some(circle) => Ok(Circle::reconstruct(
                circle.id.clone(),
                circle.name.clone(),
                circle.owner.clone(),
                circle.capacity,
                circle.members.clone(),
            )),
            None => Err(Error::msg("Circle not found")),
        }
    }

    fn create(&mut self, circle: &Circle) -> Result<(), Error> {
        match self.db.create(circle.clone()) {
            Some(_) => Ok(()),
            None => Err(Error::msg("Circle already exists")),
        }
    }

    fn save(&mut self, circle: &Circle) -> Result<(), Error> {
        match self.db.update(circle.clone()) {
            Some(_) => Ok(()),
            None => Err(Error::msg("Circle not found")),
        }
    }

    fn delete(&mut self, circle: &Circle) -> Result<(), Error> {
        match self.db.delete(&circle.id.to_string()) {
            Some(_) => Ok(()),
            None => Err(Error::msg("Circle not found")),
        }
    }
}
