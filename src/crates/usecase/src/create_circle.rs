use anyhow::Result;
use serde::Deserialize;

use domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    interface::circle_repository_interface::CircleRepositoryInterface,
};

#[derive(Debug, Deserialize)]
pub struct CreateCircleInput {
    pub circle_name: String,
    pub capacity: i16,
    pub owner_name: String,
    pub owner_age: i16,
    pub owner_grade: i16,
    pub owner_major: String,
}

impl CreateCircleInput {
    pub fn new(
        circle_name: String,
        capacity: i16,
        owner_name: String,
        owner_age: i16,
        owner_grade: i16,
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CreateCircleOutput {
    pub circle_id: String,
    pub owner_id: String,
}

pub struct CreateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> CreateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        CreateCircleUsecase { circle_repository }
    }

    pub async fn execute(
        &mut self,
        create_circle_input: CreateCircleInput,
    ) -> Result<CreateCircleOutput> {
        let grade = Grade::try_from(create_circle_input.owner_grade)?;
        let major = Major::from(create_circle_input.owner_major.as_str());
        let owner = Member::new(
            create_circle_input.owner_name,
            create_circle_input.owner_age,
            grade,
            major,
        );
        let owner_id = owner.clone().id;
        let circle = Circle::new(
            create_circle_input.circle_name,
            owner,
            create_circle_input.capacity,
        )?;
        self.circle_repository
            .create(&circle)
            .await
            .map(|_| CreateCircleOutput {
                circle_id: String::from(circle.id),
                owner_id: String::from(owner_id),
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::create_circle::{CreateCircleInput, CreateCircleOutput, CreateCircleUsecase};
    use domain::{
        aggregate::{
            circle::Circle,
            member::Member,
            value_object::{grade::Grade, major::Major},
        },
        interface::circle_repository_interface::MockCircleRepositoryInterface,
    };

    #[tokio::test]
    async fn test_create_circle_usecase() -> anyhow::Result<()> {
        let mut mocked_circle_repository = MockCircleRepositoryInterface::new();
        let grade = Grade::Third;
        let major = Major::ComputerScience;
        let owner = Member::new("mike".to_string(), 21, grade, major);
        let circle = Circle::new("music".to_string(), owner.clone(), 10)?;
        let input = CreateCircleInput::new(
            "music".to_string(),
            10,
            "mike".to_string(),
            21,
            3,
            "ComputerScience".to_string(),
        );
        let output = CreateCircleOutput {
            circle_id: circle.id.to_string(),
            owner_id: owner.id.to_string(),
        };
        println!("{:?}", owner);
        println!("{:?}", circle);

        mocked_circle_repository
            .expect_create()
            // .with(eq(circle))
            .times(1)
            .return_once(|_| Ok(()));

        let mut usecase = CreateCircleUsecase::new(mocked_circle_repository);
        let result = usecase.execute(input).await?;
        println!("{:?}", result);
        assert_eq!(result, output);
        assert_eq!(true, true);
        anyhow::Ok(())
    }
}
