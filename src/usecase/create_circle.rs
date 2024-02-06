use anyhow::Result;
use serde::Deserialize;

use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    repository::circle_repository_trait::CircleRepositoryTrait,
};

#[derive(Debug, Deserialize)]
pub struct CreateCircleInput {
    pub id: usize,
    pub circle_name: String,
    pub capacity: usize,
    pub owner_name: String,
    pub owner_age: usize,
    pub owner_grade: usize,
    pub owner_major: String,
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
        let grade = match circle_circle_input.owner_grade {
            1 => Grade::First,
            2 => Grade::Second,
            3 => Grade::Third,
            4 => Grade::Fourth,
            _ => unimplemented!("error"),
        };

        let major = match circle_circle_input.owner_major.as_str() {
            "ComputerScience" => Major::ComputerScience,
            "Economics" => Major::Economics,
            "Law" => Major::Law,
            "Art" => Major::Art,
            "Music" => Major::Music,
            _ => Major::Other,
        };
        let owner = Member::new(
            circle_circle_input.owner_name,
            circle_circle_input.owner_age,
            grade,
            major,
        );
        let circle = Circle::new(
            circle_circle_input.circle_name,
            owner,
            circle_circle_input.capacity,
        )?;
        self.circle_repository.create(&circle)
    }
}
