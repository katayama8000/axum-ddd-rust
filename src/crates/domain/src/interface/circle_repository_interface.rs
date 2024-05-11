use anyhow::Error;

use crate::aggregate::{circle::Circle, value_object::circle_id::CircleId};
pub trait CircleRepositoryInterface {
    async fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    async fn create(&self, circle: &Circle) -> Result<(), Error>;
    async fn update(&self, circle: &Circle) -> Result<Circle, Error>;
    async fn delete(&self, circle: &Circle) -> Result<(), Error>;
}
