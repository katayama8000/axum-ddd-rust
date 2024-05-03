use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::circle_repository_interface::CircleRepositoryInterface,
};

#[derive(Debug, Deserialize)]
pub struct FetchCircleInput {
    pub id: i32,
}

impl FetchCircleInput {
    pub fn new(id: i32) -> Self {
        FetchCircleInput { id }
    }
}

#[derive(Debug)]
pub struct FetchCircleOutput {
    pub circle_id: i32,
    pub circle_name: String,
    pub capacity: i32,
    pub owner: MemberOutput,
    pub members: Vec<MemberOutput>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberOutput {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub grade: i32,
    pub major: String,
}
pub struct FetchCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> FetchCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        FetchCircleUsecase { circle_repository }
    }

    pub async fn execute(
        &self,
        fetch_circle_input: FetchCircleInput,
    ) -> Result<FetchCircleOutput, Error> {
        let circle_id = CircleId::from(fetch_circle_input.id);
        self.circle_repository
            .find_circle_by_id(&circle_id)
            .await
            .map(|circle: Circle| FetchCircleOutput {
                circle_id: i32::from(circle.id),
                circle_name: circle.name,
                capacity: circle.capacity as i32,
                owner: MemberOutput {
                    id: i32::from(circle.owner.id),
                    name: circle.owner.name,
                    age: circle.owner.age,
                    grade: i32::from(circle.owner.grade),
                    major: String::from(circle.owner.major),
                },
                members: circle
                    .members
                    .iter()
                    .map(|member| MemberOutput {
                        id: i32::from(member.id),
                        name: member.name.clone(),
                        age: member.age,
                        grade: i32::from(member.grade),
                        major: String::from(member.major),
                    })
                    .collect(),
            })
    }
}
