use anyhow::Result;
use rand::Rng;

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

impl<T> CreateCircleUsecase<T>
where
    T: CircleRepositoryTrait,
{
    pub fn new(circle_repository: T) -> Self {
        CreateCircleUsecase { circle_repository }
    }

    pub fn execute(&mut self, circle_circle_input: CreateCircleInput) -> Result<()> {
        let mut rng = rand::thread_rng();
        let member_id = rng.gen::<usize>();
        let circle_id = rng.gen::<usize>();
        let owner_id = MemberId::new(member_id);
        let circle_id = CircleId::new(circle_id);
        let owner = Member::new(
            owner_id,
            circle_circle_input.owner_name,
            21,
            Grade::Third,
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
