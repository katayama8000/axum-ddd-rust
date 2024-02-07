use anyhow::Result;
use serde::Deserialize;

use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    port::circle_repository_port::CircleRepositoryPort,
};

#[derive(Debug, Deserialize)]
pub struct CreateCircleInput {
    pub circle_name: String,
    pub capacity: usize,
    pub owner_name: String,
    pub owner_age: usize,
    pub owner_grade: usize,
    pub owner_major: String,
}

impl CreateCircleInput {
    pub fn new(
        circle_name: String,
        capacity: usize,
        owner_name: String,
        owner_age: usize,
        owner_grade: usize,
        owner_major: String,
    ) -> Self {
        CreateCircleInput {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }
    }
}

pub struct CreateCircleUsecase<T>
where
    T: CircleRepositoryPort,
{
    circle_repository: T,
}

impl<T> CreateCircleUsecase<T>
where
    T: CircleRepositoryPort,
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
