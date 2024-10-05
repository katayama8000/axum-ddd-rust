use std::str::FromStr;

use anyhow::Error;
use domain::{
    aggregate::value_object::circle_id::CircleId,
    interface::circle_repository_interface::CircleRepositoryInterface,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInput {
    pub id: String,
    pub circle_name: Option<String>,
    pub capacity: Option<i16>,
}

impl UpdateCircleInput {
    pub fn new(id: String, circle_name: Option<String>, capacity: Option<i16>) -> Self {
        UpdateCircleInput {
            id,
            circle_name,
            capacity,
        }
    }
}

pub struct UpdateCircleOutPut {
    pub circle_id: String,
}

impl UpdateCircleOutPut {
    pub fn new(circle_id: String) -> Self {
        UpdateCircleOutPut { circle_id }
    }
}

pub struct UpdateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> UpdateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        UpdateCircleUsecase { circle_repository }
    }

    pub async fn execute(
        &mut self,
        update_circle_input: UpdateCircleInput,
    ) -> Result<UpdateCircleOutPut, Error> {
        let circle_id = CircleId::from_str(update_circle_input.id.as_str())?;
        let mut circle = self.circle_repository.find_by_id(&circle_id).await?;

        circle.update(
            update_circle_input.circle_name,
            update_circle_input.capacity,
        );
        self.circle_repository
            .update(&circle)
            .await
            .map(|_cirlce| UpdateCircleOutPut {
                circle_id: String::from(circle.id),
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
    async fn test_update_circle_usecase() -> anyhow::Result<()> {
        let mut mocked_circle_repository = MockCircleRepositoryInterface::new();
        let owner = Member::new("john".to_string(), 21, Grade::Third, Major::ComputerScience);
        let circle = Circle::new("music".to_string(), owner.clone(), 10)?;
        let circle_clone = circle.clone();
        mocked_circle_repository
            .expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(circle_clone.clone()));
        mocked_circle_repository
            .expect_update()
            .times(1)
            .returning(move |_| Ok(Circle::new("footBall".to_string(), owner.clone(), 20)?));
        let mut usecase = UpdateCircleUsecase::new(mocked_circle_repository);
        let input = UpdateCircleInput::new(
            circle.id.to_string(),
            Some("footBall".to_string()),
            Some(20),
        );
        let output = usecase.execute(input).await?;
        assert_eq!(output.circle_id, circle.id.to_string());
        Ok(())
    }
}
