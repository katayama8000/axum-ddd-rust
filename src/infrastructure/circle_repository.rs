use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    repository::circle_repository_trait::CirclrRepositoryTrait,
};

pub struct CircleRepository {}

impl CirclrRepositoryTrait for CircleRepository {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, ()> {
        unimplemented!()
    }

    fn create(&self, circle: &Circle) -> Result<(), ()> {
        unimplemented!()
    }

    fn save(&self, circle: &Circle) -> Result<(), ()> {
        unimplemented!()
    }

    fn delete(&self, circle: &Circle) -> Result<(), ()> {
        unimplemented!()
    }
}
