use anyhow::Error;

use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::circle_repository_interface::CircleRepositoryInterface,
};

use super::db_data::{circle_data::CircleData, member_data::MemberData};

#[derive(Clone, Debug)]
pub struct CircleRepositoryWithMySql {
    db: sqlx::MySqlPool,
}

impl CircleRepositoryWithMySql {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }
}

impl CircleRepositoryInterface for CircleRepositoryWithMySql {
    async fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error> {
        let val = sqlx::query("SELECT * FROM Circles WHERE id = ?")
            .bind(circle_id.to_string())
            .fetch_one(&self.db)
            .await
            .unwrap();
        todo!()
    }

    async fn create(&self, circle: &Circle) -> Result<(), Error> {
        todo!()
    }

    async fn update(&self, circle: &Circle) -> Result<Circle, Error> {
        todo!()
    }

    async fn delete(&self, circle: &Circle) -> Result<(), Error> {
        todo!()
    }
}

impl std::convert::From<Circle> for CircleData {
    fn from(circle: Circle) -> Self {
        Self {
            id: circle.id.into(),
            name: circle.name,
            owner_id: circle.owner.id.into(),
            capacity: circle.capacity,
            members: circle.members.into_iter().map(MemberData::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {}
