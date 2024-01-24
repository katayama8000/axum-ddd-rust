use anyhow::{Error, Result};

use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    repository::circle_repository_trait::CircleRepositoryTrait,
};

use super::input::fetch_circle::FetchCircleInput;

pub struct FetchCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    circle_repository: T,
}

impl<T: CircleRepositoryTrait> FetchCircleUsecase<T> {
    pub fn new(circle_repository: T) -> Self {
        FetchCircleUsecase { circle_repository }
    }

    pub fn execute(&self, fetch_circle_input: FetchCircleInput) -> Result<Circle, Error> {
        let circle_id = CircleId::new(fetch_circle_input.id);
        self.circle_repository.find_circle_by_id(&circle_id)
    }
}
