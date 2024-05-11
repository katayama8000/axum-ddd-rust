use anyhow::Error;

use crate::aggregate::{circle::Circle, value_object::circle_id::CircleId};
pub trait CircleRepositoryInterface {
    fn find_by_id(
        &self,
        circle_id: &CircleId,
    ) -> impl std::future::Future<Output = Result<Circle, Error>> + Send;
    fn create(
        &self,
        circle: &Circle,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn update(
        &self,
        circle: &Circle,
    ) -> impl std::future::Future<Output = Result<Circle, Error>> + Send;
    fn delete(
        &self,
        circle: &Circle,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
