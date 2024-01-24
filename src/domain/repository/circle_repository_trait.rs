use anyhow::Error;

use crate::domain::aggregate::circle::Circle;
use crate::domain::aggregate::value_object::circle_id::CircleId;

pub trait CircleRepositoryTrait {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    fn create(&self, circle: &Circle) -> Result<(), Error>;
    fn save(&self, circle: &Circle) -> Result<(), ()>;
    fn delete(&self, circle: &Circle) -> Result<(), ()>;
}
