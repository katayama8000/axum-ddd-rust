use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::circle_repository_interface::CircleRepositoryInterface,
};
use sqlx::Row;

use crate::db_schema::{circle_data::CircleData, member_data::MemberData};

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: sqlx::MySqlPool,
}

impl CircleRepository {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepository {
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
        tracing::info!("find_circle_by_id: {:?}", circle_id);

        let query = "
            SELECT 
                c.id AS circle_id, c.name AS circle_name, c.owner_id, c.capacity,
                m.id AS member_id, m.name AS member_name, m.age AS member_age, m.grade AS member_grade, m.major AS member_major
            FROM circles c
            LEFT JOIN members m ON c.id = m.circle_id
            WHERE c.id = ?
        ";

        let rows = sqlx::query(query)
            .bind(circle_id.to_string())
            .fetch_all(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch circle and members: {:?}", e);
                anyhow::Error::msg("Failed to fetch circle and members")
            })?;

        if rows.is_empty() {
            return Err(anyhow::Error::msg("Circle not found"));
        }

        let first_row = &rows[0];
        let mut members: Vec<MemberData> = Vec::new();
        let mut owner_data: Option<MemberData> = None;

        for row in &rows {
            let member = MemberData {
                id: row.get::<String, _>("member_id"),
                name: row.get::<String, _>("member_name"),
                age: row.get::<i16, _>("member_age"),
                grade: row.get::<i16, _>("member_grade"),
                major: row.get::<String, _>("member_major"),
            };

            if member.id == first_row.get::<String, _>("owner_id") {
                owner_data = Some(member.clone());
            }
            members.push(member);
        }

        let owner = owner_data.ok_or_else(|| anyhow::Error::msg("Owner not found"))?;

        let circle_data = CircleData {
            id: first_row.get::<String, _>("circle_id"),
            name: first_row.get::<String, _>("circle_name"),
            owner_id: first_row.get::<String, _>("owner_id"),
            owner,
            capacity: first_row.get::<i16, _>("capacity"),
            members,
        };

        Ok(Circle::try_from(circle_data)?)
    }

    async fn create(&self, circle: &Circle) -> Result<(), anyhow::Error> {
        tracing::info!("create_circle : {:?}", circle);
        let circle_data = CircleData::try_from(circle.clone())?;

        let mut tx = self.db.begin().await.map_err(|e| {
            eprintln!("Failed to start transaction: {:?}", e);
            anyhow::Error::msg("Failed to start transaction")
        })?;

        let circle_query =
            sqlx::query("INSERT INTO circles (id, name, owner_id, capacity) VALUES (?, ?, ?, ?)")
                .bind(circle_data.id.as_str())
                .bind(circle_data.name)
                .bind(circle_data.owner_id)
                .bind(circle_data.capacity);

        circle_query.execute(&mut *tx).await.map_err(|e| {
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
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                eprintln!("Failed to insert owner: {:?}", e);
                anyhow::Error::msg("Failed to insert owner")
            })?;

        // Commit transaction
        tx.commit().await.map_err(|e| {
            eprintln!("Failed to commit transaction: {:?}", e);
            anyhow::Error::msg("Failed to commit transaction")
        })?;
        Ok(())
    }

    async fn update(&self, circle: &Circle) -> Result<Circle, anyhow::Error> {
        tracing::info!("update_circle : {:?}", circle);
        let circle_data = CircleData::try_from(circle.clone())?;

        // Start transaction
        let mut tx = self.db.begin().await.map_err(|e| {
            eprintln!("Failed to start transaction: {:?}", e);
            anyhow::Error::msg("Failed to start transaction")
        })?;

        // Update circle
        let circle_query =
            sqlx::query("UPDATE circles SET name = ?, owner_id = ?, capacity = ? WHERE id = ?")
                .bind(circle_data.name)
                .bind(circle_data.owner_id)
                .bind(circle_data.capacity)
                .bind(circle_data.id);

        circle_query.execute(&mut *tx).await.map_err(|e| {
            eprintln!("Failed to update circle: {:?}", e);
            anyhow::Error::msg("Failed to update circle")
        })?;

        // TODO: Update members

        // Commit transaction
        tx.commit().await.map_err(|e| {
            eprintln!("Failed to commit transaction: {:?}", e);
            anyhow::Error::msg("Failed to commit transaction")
        })?;

        Ok(circle.clone())
    }

    async fn delete(&self, _circle: &Circle) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
