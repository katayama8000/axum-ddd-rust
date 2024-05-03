use crate::domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::circle_repository_interface::CircleRepositoryInterface,
};
use sqlx::Row;

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
    async fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, anyhow::Error> {
        let circle_query =
            sqlx::query("SELECT * FROM circles WHERE id = ?").bind(circle_id.to_string());

        let circle_row = circle_query.fetch_one(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch circle by id: {:?}", e);
            anyhow::Error::msg("Failed to fetch circle by id")
        })?;

        let member_query =
            sqlx::query("SELECT * FROM members WHERE circle_id = ?").bind(circle_id.to_string());

        let members_row = member_query.fetch_all(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch members by circle id: {:?}", e);
            anyhow::Error::msg("Failed to fetch members by circle id")
        })?;

        let members = members_row
            .into_iter()
            .map(|member| MemberData {
                id: member.get::<i32, _>("id"),
                name: member.get::<String, _>("name"),
                age: member.get::<i32, _>("age"),
                grade: member.get::<i32, _>("grade"),
                major: member.get::<String, _>("major"),
            })
            .collect();

        let circle_data = CircleData {
            id: circle_row.get::<i32, _>("id"),
            name: circle_row.get::<String, _>("name"),
            owner_id: circle_row.get::<i32, _>("owner_id"),
            capacity: circle_row.get::<i32, _>("capacity"),
            members,
        };

        Ok(Circle::try_from(circle_data)?)
    }

    async fn create(&self, circle: &Circle) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn update(&self, circle: &Circle) -> Result<Circle, anyhow::Error> {
        todo!()
    }

    async fn delete(&self, circle: &Circle) -> Result<(), anyhow::Error> {
        todo!()
    }
}

impl std::convert::From<Circle> for CircleData {
    fn from(circle: Circle) -> Self {
        Self {
            id: circle.id.into(),
            name: circle.name,
            owner_id: circle.owner.id.into(),
            capacity: circle.capacity as i32,
            members: circle.members.into_iter().map(MemberData::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {}
