use anyhow::{Error, Result};
use serde::Deserialize;

use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    repository::circle_repository_trait::CircleRepositoryTrait,
};

#[derive(Debug, Deserialize)]
pub struct FetchCircleInput {
    pub id: usize,
}

pub struct FetchCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    circle_repository: T,
}

impl<T> FetchCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    pub fn new(circle_repository: T) -> Self {
        FetchCircleUsecase { circle_repository }
    }

    pub fn execute(&self, fetch_circle_input: FetchCircleInput) -> Result<Circle, Error> {
        let circle_id = CircleId::new(fetch_circle_input.id);
        self.circle_repository.find_circle_by_id(&circle_id)
    }
}
