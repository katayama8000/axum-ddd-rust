use domain::{
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

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepositoryWithMySql {
    async fn find_all(&self) -> Result<Vec<Circle>, anyhow::Error> {
        tracing::info!("find_all_circles");
        let circle_query = sqlx::query("SELECT * FROM circles");

        let circle_rows = circle_query.fetch_all(&self.db).await.map_err(|e| {
            eprintln!("Failed to fetch circles: {:?}", e);
            anyhow::Error::msg("Failed to fetch circles")
        })?;

        let mut circles = Vec::new();
        for circle_row in circle_rows {
            let member_query = sqlx::query("SELECT * FROM members WHERE circle_id = ?")
                .bind(circle_row.get::<i16, _>("id"));

            let members_row = member_query.fetch_all(&self.db).await.map_err(|e| {
                eprintln!("Failed to fetch members by circle id: {:?}", e);
                anyhow::Error::msg("Failed to fetch members by circle id")
            })?;

            let members: Vec<MemberData> = members_row
                .into_iter()
                .map(|member| MemberData {
                    id: member.get::<String, _>("id"),
                    name: member.get::<String, _>("name"),
                    age: member.get::<i16, _>("age"),
                    grade: member.get::<i16, _>("grade"),
                    major: member.get::<String, _>("major"),
                })
                .collect();

            let owner: MemberData = members
                .iter()
                .find(|member| member.id == circle_row.get::<String, _>("owner_id"))
                .ok_or_else(|| anyhow::Error::msg("Owner not found"))?
                .clone();

            let circle_data = CircleData {
                id: circle_row.get::<String, _>("id"),
                name: circle_row.get::<String, _>("name"),
                owner_id: circle_row.get::<String, _>("owner_id"),
                owner,
                capacity: circle_row.get::<i16, _>("capacity"),
                members,
            };

            circles.push(Circle::try_from(circle_data)?);
        }

        Ok(circles)
    }

    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, anyhow::Error> {
        tracing::info!("find_circle_by_id : {:?}", circle_id);
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

        let members: Vec<MemberData> = members_row
            .into_iter()
            .map(|member| MemberData {
                id: member.get::<String, _>("id"),
                name: member.get::<String, _>("name"),
                age: member.get::<i16, _>("age"),
                grade: member.get::<i16, _>("grade"),
                major: member.get::<String, _>("major"),
            })
            .collect();

        let owner: MemberData = members
            .iter()
            .find(|member| member.id == circle_row.get::<String, _>("owner_id"))
            .ok_or_else(|| anyhow::Error::msg("Owner not found"))?
            .clone();

        let circle_data = CircleData {
            id: circle_row.get::<String, _>("id"),
            name: circle_row.get::<String, _>("name"),
            owner_id: circle_row.get::<String, _>("owner_id"),
            owner,
            capacity: circle_row.get::<i16, _>("capacity"),
            members,
        };

        Ok(Circle::try_from(circle_data)?)
    }

    async fn create(&self, circle: &Circle) -> Result<(), anyhow::Error> {
        tracing::info!("create_circle : {:?}", circle);
        let circle_data = CircleData::try_from(circle.clone())?;
        let circle_query =
            sqlx::query("INSERT INTO circles (id, name, owner_id, capacity) VALUES (?, ?, ?, ?)")
                .bind(circle_data.id.as_str())
                .bind(circle_data.name)
                .bind(circle_data.owner_id)
                .bind(circle_data.capacity);

        circle_query.execute(&self.db).await.map_err(|e| {
            eprintln!("Failed to insert circle: {:?}", e);
            anyhow::Error::msg("Failed to insert circle")
        })?;

        let owner_query = sqlx::query(
            "INSERT INTO members (id, name, age, grade, major, circle_id) VALUES (?, ?, ?, ?, ?, ?)",
        );

        owner_query
            .bind(circle_data.owner.id)
            .bind(circle_data.owner.name)
            .bind(circle_data.owner.age)
            .bind(circle_data.owner.grade)
            .bind(circle_data.owner.major)
            .bind(circle_data.id.as_str())
            .execute(&self.db)
            .await
            .map_err(|e| {
                eprintln!("Failed to insert owner: {:?}", e);
                anyhow::Error::msg("Failed to insert owner")
            })?;

        if circle_data.members.is_empty() {
            return Ok(());
        }
        Ok(())
    }

    async fn update(&self, circle: &Circle) -> Result<Circle, anyhow::Error> {
        tracing::info!("update_circle : {:?}", circle);
        let circle_data = CircleData::try_from(circle.clone())?;
        let circle_query =
            sqlx::query("UPDATE circles SET name = ?, owner_id = ?, capacity = ? WHERE id = ?")
                .bind(circle_data.name)
                .bind(circle_data.owner_id)
                .bind(circle_data.capacity)
                .bind(circle_data.id);

        circle_query.execute(&self.db).await.map_err(|e| {
            eprintln!("Failed to update circle: {:?}", e);
            anyhow::Error::msg("Failed to update circle")
        })?;

        // let member_query =
        //     sqlx::query("DELETE FROM members WHERE circle_id = ?").bind(circle_data.id);
        // member_query.execute(&self.db).await.map_err(|e| {
        //     eprintln!("Failed to delete members: {:?}", e);
        //     anyhow::Error::msg("Failed to delete members")
        // })?;

        // for member in circle_data.members {
        //     let member_query = sqlx::query(
        //         "INSERT INTO members (name, age, grade, major, circle_id) VALUES (?, ?, ?, ?, ?, ?)",
        //     );
        //     member_query
        //         .bind(member.name)
        //         .bind(member.age)
        //         .bind(member.grade)
        //         .bind(member.major)
        //         .bind(circle_data.id)
        //         .execute(&self.db)
        //         .await
        //         .map_err(|e| {
        //             eprintln!("Failed to insert member: {:?}", e);
        //             anyhow::Error::msg("Failed to insert member")
        //         })?;
        // }

        Ok(circle.clone())
    }

    async fn delete(&self, _circle: &Circle) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
