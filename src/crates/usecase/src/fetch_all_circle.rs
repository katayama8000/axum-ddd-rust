use anyhow::{Error, Ok, Result};
use serde::Deserialize;

use domain::interface::circle_repository_interface::CircleRepositoryInterface;

#[derive(Debug, Deserialize)]
pub struct FetchAllCircleInput {
    pub id: i16,
}

impl FetchAllCircleInput {
    pub fn new(id: i16) -> Self {
        FetchAllCircleInput { id }
    }
}

// TODO: Define the output struct
#[derive(Debug)]
pub struct FetchAllCircleOutput {}

pub struct FetchAllCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> FetchAllCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        FetchAllCircleUsecase { circle_repository }
    }

    pub async fn execute(&self) -> Result<FetchAllCircleOutput, Error> {
        self.circle_repository.find_all().await?;
        Ok(FetchAllCircleOutput {})
    }
}
