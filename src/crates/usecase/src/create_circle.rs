use anyhow::Result;
use serde::Deserialize;

use domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    interface::{
        circle_duplicate_checker_interface::CircleDuplicateCheckerInterface,
        circle_repository_interface::CircleRepositoryInterface,
    },
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

pub struct CreateCircleUsecase<T, U>
where
    T: CircleRepositoryInterface,
    U: CircleDuplicateCheckerInterface,
{
    circle_repository: T,
    circle_duplicate_checker: U,
}

impl<T, U> CreateCircleUsecase<T, U>
where
    T: CircleRepositoryInterface,
    U: CircleDuplicateCheckerInterface,
{
    pub fn new(circle_repository: T, circle_duplicate_checker: U) -> Self {
        CreateCircleUsecase {
            circle_repository,
            circle_duplicate_checker,
        }
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
        let owner_id = owner.id.clone();
        let circle = Circle::new(
            create_circle_input.circle_name,
            owner,
            create_circle_input.capacity,
        )?;
        self.circle_duplicate_checker
            .check_circle_duplicate(&circle)
            .await?;
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
    use super::*;
    use anyhow::anyhow;
    use domain::interface::{
        circle_duplicate_checker_interface::MockCircleDuplicateCheckerInterface,
        circle_repository_interface::MockCircleRepositoryInterface,
    };

    #[tokio::test]
    async fn test_create_circle_usecase_successful() -> anyhow::Result<()> {
        let mut mocked_circle_repository = MockCircleRepositoryInterface::new();
        let mut mocked_circle_duplicate_checker = MockCircleDuplicateCheckerInterface::new();

        let input = CreateCircleInput::new(
            "music".to_string(),
            10,
            "mike".to_string(),
            21,
            3,
            "ComputerScience".to_string(),
        );

        mocked_circle_repository
            .expect_create()
            .times(1)
            .return_once(|_| Ok(()));
        mocked_circle_duplicate_checker
            .expect_check_circle_duplicate()
            .times(1)
            .return_once(|_| Ok(()));

        let mut usecase =
            CreateCircleUsecase::new(mocked_circle_repository, mocked_circle_duplicate_checker);
        let _result = usecase.execute(input).await?;

        anyhow::Ok(())
    }

    #[tokio::test]
    async fn test_create_circle_usecase_duplicate_error() -> anyhow::Result<()> {
        let mut mocked_circle_repository = MockCircleRepositoryInterface::new();
        let mut mocked_circle_duplicate_checker = MockCircleDuplicateCheckerInterface::new();

        let input = CreateCircleInput::new(
            "music".to_string(),
            10,
            "mike".to_string(),
            21,
            3,
            "ComputerScience".to_string(),
        );

        mocked_circle_duplicate_checker
            .expect_check_circle_duplicate()
            .times(1)
            .return_once(|_| Err(anyhow!("Circle name already exists")));

        mocked_circle_repository.expect_create().times(0);

        let mut usecase =
            CreateCircleUsecase::new(mocked_circle_repository, mocked_circle_duplicate_checker);
        let result = usecase.execute(input).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Circle name already exists"
        );

        anyhow::Ok(())
    }
}
