use anyhow::Error;

use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    repository::circle_repository_trait::CircleRepositoryTrait,
};

pub struct CircleRepository {}

impl CircleRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl CircleRepositoryTrait for CircleRepository {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error> {
        unimplemented!()
    }

    fn create(&self, circle: &Circle) -> Result<(), Error> {
        unimplemented!()
    }

    fn save(&self, circle: &Circle) -> Result<(), ()> {
        unimplemented!()
    }

    fn delete(&self, circle: &Circle) -> Result<(), ()> {
        unimplemented!()
    }
}
