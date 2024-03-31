use serde::Deserialize;

use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    interface::circle_repository_interface::CircleRepositoryInterface,
};

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CreateCircleInput {
    pub circle_name: String,
    pub capacity: usize,
    pub owner_name: String,
    pub owner_age: usize,
    pub owner_grade: usize,
    pub owner_major: String,
}

impl CreateCircleInput {
    pub fn new(
        circle_name: String,
        capacity: usize,
        owner_name: String,
        owner_age: usize,
        owner_grade: usize,
        owner_major: String,
    ) -> Self {
        CreateCircleInput {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CreateCircleOutput {
    pub circle_id: usize,
    pub owner_id: usize,
}

pub struct CreateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> CreateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        CreateCircleUsecase { circle_repository }
    }

    pub fn execute(
        &mut self,
        circle_circle_input: CreateCircleInput,
    ) -> anyhow::Result<CreateCircleOutput> {
        let grade = Grade::try_from(circle_circle_input.owner_grade)?;

        let major = Major::from(circle_circle_input.owner_major.as_str());

        let owner = Member::new(
            circle_circle_input.owner_name,
            circle_circle_input.owner_age,
            grade,
            major,
        );
        let owner_id = owner.id;
        let circle = Circle::new(
            circle_circle_input.circle_name,
            owner,
            circle_circle_input.capacity,
        )?;
        self.circle_repository
            .create(&circle)
            .map(|_| CreateCircleOutput {
                circle_id: usize::from(circle.id),
                owner_id: usize::from(owner_id),
            })
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::domain::aggregate::circle::Circle;
    use crate::domain::interface::circle_repository_interface::MockCircleRepositoryInterface;

    #[test]
    fn test_create_circle_usecase() -> anyhow::Result<()> {
        let mut mocked_circle_repository = MockCircleRepositoryInterface::new();
        let owner = Member::sample(
            "John Lennon".to_string(),
            20,
            Grade::Third,
            Major::ComputerScience,
        );
        let circle = Circle::sample("Football Circle".to_string(), owner, 10)?;
        println!("{:?}", circle);
        let input = CreateCircleInput::new(
            "Football Circle".to_string(),
            10,
            "John Lennon".to_string(),
            20,
            3,
            "ComputerScience".to_string(),
        );
        let output = CreateCircleOutput {
            circle_id: 1,
            owner_id: 1,
        };

        mocked_circle_repository
            .expect_create()
            .with(eq(circle))
            .times(1)
            .return_once(|_| Ok(()));

        let mut usecase = CreateCircleUsecase::new(mocked_circle_repository);
        let result = usecase.execute(input)?;
        assert_eq!(result, output);
        assert_eq!(true, true);
        anyhow::Ok(())
    }
}
