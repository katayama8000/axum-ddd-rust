use anyhow::{Error, Result};
use serde::Deserialize;

use crate::domain::{
    aggregate::{circle::Circle, member::Member, value_object::circle_id::CircleId},
    port::circle_repository_port::CircleRepositoryPort,
};

#[derive(Debug, Deserialize)]
pub struct FetchCircleInput {
    pub id: usize,
}

impl FetchCircleInput {
    pub fn new(id: usize) -> Self {
        FetchCircleInput { id }
    }
}

#[derive(Debug, Deserialize)]
pub struct FetchCircleOutput {
    pub circle_id: usize,
    pub circle_name: String,
    pub capacity: usize,
    pub owner: Member,
    pub members: Vec<Member>,
}
pub struct FetchCircleUsecase<T>
where
    T: CircleRepositoryPort,
{
    circle_repository: T,
}

impl<T> FetchCircleUsecase<T>
where
    T: CircleRepositoryPort,
{
    pub fn new(circle_repository: T) -> Self {
        FetchCircleUsecase { circle_repository }
    }

    pub fn execute(
        &self,
        fetch_circle_input: FetchCircleInput,
    ) -> Result<FetchCircleOutput, Error> {
        let circle_id = CircleId::new(fetch_circle_input.id);
        self.circle_repository
            .find_circle_by_id(&circle_id)
            .map(|circle: Circle| FetchCircleOutput {
                circle_id: usize::from(circle.id),
                circle_name: circle.name,
                capacity: circle.capacity,
                owner: circle.owner,
                members: circle.members,
            })
    }
}
