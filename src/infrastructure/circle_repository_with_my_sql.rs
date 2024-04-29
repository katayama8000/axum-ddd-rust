use anyhow::Error;

use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
    },
    interface::circle_repository_interface::CircleRepositoryInterface,
};

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

#[derive(serde::Deserialize, serde::Serialize)]
struct CircleData {
    id: Vec<u8>, // BINARY(16)に対応するバイト配列型に修正
    name: String,
    owner_id: Vec<u8>, // BINARY(16)に対応するバイト配列型に修正
    capacity: usize,
    members: Vec<MemberData>,
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

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MemberData {
    id: usize,
    name: String,
    age: usize,
    grade: usize,
    major: String,
}

impl std::convert::From<Member> for MemberData {
    fn from(value: Member) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            age: value.age,
            grade: value.grade.into(),
            major: value.major.into(),
        }
    }
}

impl std::convert::TryFrom<MemberData> for Member {
    type Error = Error;

    fn try_from(value: MemberData) -> Result<Self, Self::Error> {
        Ok(Member::reconstruct(
            MemberId::from(value.id),
            value.name,
            value.age,
            Grade::try_from(value.grade)?,
            Major::from(value.major.as_str()),
        ))
    }
}

#[cfg(test)]
mod tests {}
