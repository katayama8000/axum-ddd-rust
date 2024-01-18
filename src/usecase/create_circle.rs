use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
    },
    repository::circle_repository_trait::CirclrRepositoryTrait,
};

pub struct CreateCircleInput {
    pub id: usize,
    pub circle_name: String,
    pub owner_name: String,
    pub capacity: usize,
}

pub fn execute<T>(port: T, circle_circle_input: CreateCircleInput) -> Result<(), ()>
where
    T: CirclrRepositoryTrait,
{
    let member_id = MemberId::new(1);
    let circle_id = CircleId::new(1);
    let owner = Member::new(
        member_id,
        circle_circle_input.owner_name,
        21,
        Grade::Fourth,
        Major::Art,
    );
    let circle = Circle::new(
        circle_id,
        circle_circle_input.circle_name,
        owner,
        circle_circle_input.capacity,
    );
    port.create(&circle)?;
    Ok(())
}
