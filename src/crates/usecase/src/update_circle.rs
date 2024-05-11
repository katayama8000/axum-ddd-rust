use anyhow::Error;
use domain::{
    aggregate::value_object::circle_id::CircleId,
    interface::circle_repository_interface::CircleRepositoryInterface,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInput {
    pub id: i16,
    pub circle_name: Option<String>,
    pub capacity: Option<i16>,
}

impl UpdateCircleInput {
    pub fn new(id: i16, circle_name: Option<String>, capacity: Option<i16>) -> Self {
        UpdateCircleInput {
            id,
            circle_name,
            capacity,
        }
    }
}

pub struct UpdateCircleOutPut {
    pub id: i16,
}

impl UpdateCircleOutPut {
    pub fn new(id: i16) -> Self {
        UpdateCircleOutPut { id }
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
        let circle_id = CircleId::from(update_circle_input.id);
        let mut circle = self.circle_repository.find_by_id(&circle_id).await?;

        circle.update(
            update_circle_input.circle_name,
            update_circle_input.capacity,
        );
        self.circle_repository
            .update(&circle)
            .await
            .map(|_cirlce| UpdateCircleOutPut {
                id: i16::from(circle.id),
            })
    }
}
