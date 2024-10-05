use anyhow::{Error, Ok, Result};
use serde::Deserialize;

use domain::interface::circle_repository_interface::CircleRepositoryInterface;

#[derive(Debug, Deserialize)]
pub struct FetchAllCircleInput {
    pub id: i16,
}

impl FetchAllCircleInput {
    pub fn new(id: i16) -> Self {
        FetchAllCircleInput { id }
    }
}

// TODO: Define the output struct
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct FetchAllCircleOutput {}

pub struct FetchAllCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> FetchAllCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        FetchAllCircleUsecase { circle_repository }
    }

    pub async fn execute(&self) -> Result<FetchAllCircleOutput, Error> {
        self.circle_repository.find_all().await?;
        Ok(FetchAllCircleOutput {})
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
    async fn test_fetch_all_circle_usecase() -> anyhow::Result<()> {
        let mut mocked_circle_repository = MockCircleRepositoryInterface::new();
        let owner = Member::new(
            "john".to_string(),
            21,
            Grade::try_from(3)?,
            Major::from("ComputerScience"),
        );
        let circle = Circle::new("music".to_string(), owner.clone(), 10)?;

        mocked_circle_repository
            .expect_find_all()
            .returning(move || Ok(vec![circle.clone()]));

        let usecase = FetchAllCircleUsecase::new(mocked_circle_repository);
        let output = usecase.execute().await.unwrap();

        assert_eq!(output, FetchAllCircleOutput {});
        Ok(())
    }
}
