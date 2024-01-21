use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
    },
    repository::circle_repository_trait::CircleRepositoryTrait,
};

pub struct FetchCircleInput {
    pub id: usize,
}

pub struct FetchCircleService<T>
where
    T: CircleRepositoryTrait,
{
    circle_repository: T,
}

impl<T: CircleRepositoryTrait> FetchCircleService<T> {
    pub fn new(circle_repository: T) -> Self {
        FetchCircleService { circle_repository }
    }

    pub fn execute(&self, fetch_circle_input: FetchCircleInput) {
        let circle_id = CircleId::new(fetch_circle_input.id);
        match self.circle_repository.find_circle_by_id(&circle_id) {
            Ok(_) => println!("success"),
            Err(_) => println!("error"),
        }
    }
}
