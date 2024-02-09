use anyhow::Error;
use serde::Deserialize;

use crate::domain::{
    aggregate::value_object::circle_id::CircleId,
    port::circle_repository_port::CircleRepositoryPort,
};

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInput {
    pub id: usize,
    pub circle_name: Option<String>,
    pub capacity: Option<usize>,
}

impl UpdateCircleInput {
    pub fn new(id: usize, circle_name: Option<String>, capacity: Option<usize>) -> Self {
        UpdateCircleInput {
            id,
            circle_name,
            capacity,
        }
    }
}

pub struct UpdateCircleUsecase<T>
where
    T: CircleRepositoryPort,
{
    circle_repository: T,
}

impl<T> UpdateCircleUsecase<T>
where
    T: CircleRepositoryPort,
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
        self.circle_repository.update(&circle)
    }
}
