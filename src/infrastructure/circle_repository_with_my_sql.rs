use std::vec;

use anyhow::Error;

use crate::domain::{
    aggregate::{circle::Circle, member, value_object::circle_id::CircleId},
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
    async fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error> {
        println!("circle_id: {:?}", circle_id.to_string());
        let circle = sqlx::query("SELECT * FROM circles WHERE id = ?")
            .bind(circle_id.to_string())
            .fetch_one(&self.db)
            .await
            .expect("Failed to fetch circle by id");

        let id = circle.get::<i32, _>("id");
        let name = circle.get::<String, _>("name");
        let capacity = circle.get::<i32, _>("capacity");
        let owner_id = circle.get::<i32, _>("owner_id");

        println!("id: {:?}", id);
        println!("name: {:?}", name);
        println!("capacity: {:?}", capacity);
        println!("owner_id: {:?}", owner_id);

        let members = sqlx::query("SELECT * FROM members WHERE circle_id = ?")
            .bind(circle_id.to_string())
            .fetch_all(&self.db)
            .await
            .expect("Failed to fetch members by circle id");

        // for member in members {
        //     let id = member.get::<i32, _>("id");

        //     let name = member.get::<String, _>("name");
        //     let grade = member.get::<i32, _>("grade");

        //     println!("id: {:?}", id);
        //     println!("name: {:?}", name);
        //     println!("grade: {:?}", grade);
        // }

        let mut members_data: Vec<MemberData> = vec![];
        for member in members {
            let id = member.get::<i32, _>("id");

            let name = member.get::<String, _>("name");
            let grade = member.get::<i32, _>("grade");
            let age = 10;
            let major = "Computer Science".to_string();

            members_data.extend(vec![MemberData {
                id,
                name,
                age,
                grade,
                major,
            }])
        }

        let circle_data = CircleData {
            id,
            name,
            owner_id,
            capacity,
            members: members_data,
        };

        Ok(Circle::try_from(circle_data)?)
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
            capacity: circle.capacity as i32,
            members: circle.members.into_iter().map(MemberData::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {}
