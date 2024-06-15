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
