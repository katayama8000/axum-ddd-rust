use anyhow::Error;

use crate::domain::aggregate::circle::Circle;
use crate::domain::aggregate::value_object::circle_id::CircleId;

pub trait CircleRepositoryTrait {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    fn create(&mut self, circle: &Circle) -> Result<(), Error>;
    fn save(&mut self, circle: &Circle) -> Result<(), Error>;
    fn delete(&mut self, circle: &Circle) -> Result<(), Error>;
}
