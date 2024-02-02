use anyhow::Result;

use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
    },
    repository::circle_repository_trait::CircleRepositoryTrait,
};

pub struct CreateCircleInput {
    pub id: usize,
    pub circle_name: String,
    pub owner_name: String,
    pub capacity: usize,
}

pub struct CreateCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    circle_repository: T,
}

impl<T: CircleRepositoryTrait> CreateCircleUsecase<T> {
    pub fn new(circle_repository: T) -> Self {
        CreateCircleUsecase { circle_repository }
    }

    pub fn execute(&mut self, circle_circle_input: CreateCircleInput) -> Result<()> {
        let member_id = MemberId::new(1);
        let circle_id = CircleId::new(1);
        let owner = Member::new(
            member_id,
            circle_circle_input.owner_name,
            21,
            Grade::Fourth,
            Major::Art,
        );
        let circle = Circle::new(
            circle_id,
            circle_circle_input.circle_name,
            owner,
            circle_circle_input.capacity,
        )
        .unwrap();
        self.circle_repository.create(&circle)
    }
}
