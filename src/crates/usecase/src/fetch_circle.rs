use std::str::FromStr;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::circle_repository_interface::CircleRepositoryInterface,
};

#[derive(Debug, Deserialize)]
pub struct FetchCircleInput {
    pub id: String,
}

impl FetchCircleInput {
    pub fn new(id: String) -> Self {
        FetchCircleInput { id }
    }
}

#[derive(Debug)]
pub struct FetchCircleOutput {
    pub circle_id: String,
    pub circle_name: String,
    pub capacity: i16,
    pub owner: MemberOutput,
    pub members: Vec<MemberOutput>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberOutput {
    pub id: String,
    pub name: String,
    pub age: i16,
    pub grade: i16,
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
        let circle_id = CircleId::from_str(fetch_circle_input.id.as_str())?;
        self.circle_repository
            .find_by_id(&circle_id)
            .await
            .map(|circle: Circle| FetchCircleOutput {
                circle_id: circle.id.into(),
                circle_name: circle.name,
                capacity: circle.capacity as i16,
                owner: MemberOutput {
                    id: String::from(circle.owner.id),
                    name: circle.owner.name,
                    age: circle.owner.age,
                    grade: i16::from(circle.owner.grade),
                    major: String::from(circle.owner.major),
                },
                members: circle
                    .members
                    .iter()
                    .map(|member| MemberOutput {
                        id: member.id.clone().into(),
                        name: member.name.clone(),
                        age: member.age,
                        grade: i16::from(member.grade),
                        major: String::from(member.major),
                    })
                    .collect(),
            })
    }
}

#[cfg(test)]
mod tests {

    use domain::{
        aggregate::{
            circle::Circle,
            member::Member,
            value_object::{grade::Grade, major::Major},
        },
        interface::circle_repository_interface::MockCircleRepositoryInterface,
    };

    use super::*;

    #[tokio::test]
    async fn test_fetch_circle_usecase() -> anyhow::Result<()> {
        let mut mocked_circle_repository = MockCircleRepositoryInterface::new();
        let circle_id = CircleId::from_str("123")?;
        let owner = Member::new(
            "john".to_string(),
            21,
            Grade::try_from(3)?,
            Major::from("ComputerScience"),
        );
        let members = vec![Member::new(
            "mike".to_string(),
            19,
            Grade::try_from(1).unwrap(),
            Major::from("Economics"),
        )];
        let circle = Circle::new("music".to_string(), owner.clone(), 10)?;
        let circle = Circle::reconstruct(
            circle_id.clone(),
            circle.name,
            circle.owner,
            circle.capacity,
            members.clone(),
        );

        mocked_circle_repository
            .expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(circle.clone()));

        let usecase = FetchCircleUsecase::new(mocked_circle_repository);
        let input = FetchCircleInput::new(circle_id.clone().into());
        let output = usecase.execute(input).await.unwrap();

        assert_eq!(output.circle_id, circle_id.to_string());
        assert_eq!(output.circle_name, "music");
        assert_eq!(output.capacity, 10);
        assert_eq!(output.owner.id, owner.id.to_string());
        assert_eq!(output.owner.name, "john");
        assert_eq!(output.owner.age, 21);
        assert_eq!(output.owner.grade, 3);
        assert_eq!(output.owner.major, "ComputerScience");
        assert_eq!(output.members.len(), 1);
        assert_eq!(output.members[0].id, members[0].id.clone().to_string());
        assert_eq!(output.members[0].name, "mike");
        assert_eq!(output.members[0].age, 19);
        assert_eq!(output.members[0].grade, 1);
        assert_eq!(output.members[0].major, "Economics");
        Ok(())
    }
}
