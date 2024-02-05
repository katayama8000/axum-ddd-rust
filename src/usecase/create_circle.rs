use anyhow::Result;

use crate::domain::{
    aggregate::{circle::Circle, member::Member},
    repository::circle_repository_trait::CircleRepositoryTrait,
};

pub struct CreateCircleInput {
    pub id: usize,
    pub circle_name: String,
    pub owner: Member,
    pub capacity: usize,
}

pub struct CreateCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    circle_repository: T,
}

impl<T> CreateCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    pub fn new(circle_repository: T) -> Self {
        CreateCircleUsecase { circle_repository }
    }

    pub fn execute(&mut self, circle_circle_input: CreateCircleInput) -> Result<()> {
        let owner = Member::new(
            circle_circle_input.owner.name,
            circle_circle_input.owner.age,
            circle_circle_input.owner.grade,
            circle_circle_input.owner.major,
        );
        let circle = Circle::new(
            circle_circle_input.circle_name,
            owner,
            circle_circle_input.capacity,
        )?;
        self.circle_repository.create(&circle)
    }
}
