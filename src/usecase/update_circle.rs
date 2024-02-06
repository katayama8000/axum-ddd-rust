use anyhow::Error;
use serde::Deserialize;

use crate::domain::{
    aggregate::value_object::circle_id::CircleId,
    repository::circle_repository_trait::CircleRepositoryTrait,
};

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInput {
    pub id: usize,
    pub circle_name: Option<String>,
    pub capacity: Option<usize>,
}

pub struct UpdateCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    circle_repository: T,
}

impl<T> UpdateCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    pub fn new(circle_repository: T) -> Self {
        UpdateCircleUsecase { circle_repository }
    }

    pub fn execute(&mut self, update_circle_input: UpdateCircleInput) -> Result<(), Error> {
        let circle_id = CircleId::new(update_circle_input.id);
        let mut circle = self.circle_repository.find_circle_by_id(&circle_id)?;

        circle.update(
            update_circle_input.circle_name,
            update_circle_input.capacity,
        );
        self.circle_repository.save(&circle)
    }
}
