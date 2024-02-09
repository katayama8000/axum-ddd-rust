use anyhow::Error;

use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    port::circle_repository_port::CircleRepositoryPort,
};

use super::db::Db;

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: Db,
}

impl CircleRepository {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

impl CircleRepositoryPort for CircleRepository {
    fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error> {
        match self.db.find(&circle_id.to_string()) {
            Some(circle) => Ok(Circle::reconstruct(
                circle.id.clone(),
                circle.name.clone(),
                circle.owner.clone(),
                circle.capacity,
                circle.members.clone(),
            )),
            None => Err(Error::msg("Circle not found")),
        }
    }

    fn create(&self, circle: &Circle) -> Result<(), Error> {
        match self.db.create(circle.clone()) {
            Some(_) => Err(Error::msg("Circle already exists")),
            None => Ok(()),
        }
    }

    fn update(&self, circle: &Circle) -> Result<(), Error> {
        match self.db.update(circle.clone()) {
            Some(_) => Ok(()),
            None => Err(Error::msg("Circle not found")),
        }
    }

    fn delete(&self, circle: &Circle) -> Result<(), Error> {
        match self.db.delete(&circle.id.to_string()) {
            Some(_) => Ok(()),
            None => Err(Error::msg("Circle not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        aggregate::{
            circle::Circle,
            member::Member,
            value_object::{grade::Grade, major::Major},
        },
        port::circle_repository_port::CircleRepositoryPort,
    };

    use super::CircleRepository;

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut circle1 = build_circle()?;
        let repository = CircleRepository::new();
        assert!(repository.find_circle_by_id(&circle1.id).is_err());
        repository.create(&circle1)?;
        assert_eq!(repository.find_circle_by_id(&circle1.id)?, circle1);
        circle1.name = "circle_name2".to_string();
        repository.update(&circle1)?;
        assert_eq!(repository.find_circle_by_id(&circle1.id)?, circle1);
        repository.delete(&circle1)?;
        assert!(repository.find_circle_by_id(&circle1.id).is_err());
        Ok(())
    }

    fn build_circle() -> anyhow::Result<Circle> {
        Circle::new(
            "circle_name1".to_string(),
            Member::new("member_name1".to_string(), 21, Grade::Third, Major::Art),
            3,
        )
    }
}
