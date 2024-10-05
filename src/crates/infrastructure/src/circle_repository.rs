use std::str::FromStr;

use anyhow::Error;
use domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
    },
    interface::circle_repository_interface::CircleRepositoryInterface,
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

#[async_trait::async_trait]
impl CircleRepositoryInterface for CircleRepository {
    async fn find_all(&self) -> Result<Vec<Circle>, Error> {
        todo!("Implement this method")
    }

    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error> {
        match self.db.get::<CircleData, _>(&circle_id.to_string())? {
            Some(data) => Ok(Circle::try_from(data)?),
            None => Err(Error::msg("Circle not found")),
        }
    }

    async fn create(&self, circle: &Circle) -> Result<(), Error> {
        match self.db.get::<CircleData, _>(&circle.id.to_string())? {
            Some(_) => Err(Error::msg("Circle already exists")),
            None => {
                self.db
                    .set(circle.id.to_string(), &CircleData::from(circle.clone()))?;
                Ok(())
            }
        }
    }

    async fn update(&self, circle: &Circle) -> Result<Circle, Error> {
        match self.db.get::<CircleData, _>(&circle.id.to_string())? {
            Some(_) => self
                .db
                .set(circle.id.to_string(), &CircleData::from(circle.clone()))
                .and_then(|_| self.db.get::<CircleData, _>(&circle.id.to_string()))
                .map(|data| match data {
                    Some(data) => Circle::try_from(data),
                    None => Err(Error::msg("Failed to convert circle data")),
                })?,
            None => Err(Error::msg("Circle not found")),
        }
    }

    async fn delete(&self, circle: &Circle) -> Result<(), Error> {
        match self.db.get::<CircleData, _>(&circle.id.to_string())? {
            Some(_) => self.db.remove(circle.id.to_string()),
            None => Err(Error::msg("Circle not found")),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CircleData {
    id: String,
    name: String,
    owner: MemberData,
    capacity: i16,
    members: Vec<MemberData>,
}

impl std::convert::From<Circle> for CircleData {
    fn from(circle: Circle) -> Self {
        CircleData {
            id: circle.id.into(),
            name: circle.name,
            owner: MemberData::from(circle.owner),
            capacity: circle.capacity,
            members: circle.members.into_iter().map(MemberData::from).collect(),
        }
    }
}

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
        Ok(Circle::reconstruct(
            CircleId::from_str(&data.id)?,
            data.name,
            Member::reconstruct(
                MemberId::from_str(&data.owner.id)?,
                data.owner.name,
                data.owner.age,
                Grade::try_from(data.owner.grade)?,
                Major::from(data.owner.major.as_str()),
            ),
            data.capacity,
            data.members
                .into_iter()
                .map(Member::try_from)
                .collect::<Result<Vec<Member>, Error>>()?,
        ))
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MemberData {
    id: String,
    name: String,
    age: i16,
    grade: i16,
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
            MemberId::from_str(value.id.as_str())?,
            value.name,
            value.age,
            Grade::try_from(value.grade)?,
            Major::from(value.major.as_str()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use domain::{
        aggregate::{
            circle::Circle,
            member::Member,
            value_object::{grade::Grade, major::Major},
        },
        interface::circle_repository_interface::CircleRepositoryInterface,
    };

    use super::CircleRepository;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let mut circle1 = build_circle()?;
        let repository = CircleRepository::new();
        assert!(repository.find_by_id(&circle1.id).await.is_err());
        repository.create(&circle1).await?;
        assert_eq!(repository.find_by_id(&circle1.id).await?, circle1);
        circle1.name = "circle_name2".to_string();
        repository.update(&circle1).await?;
        assert_eq!(repository.find_by_id(&circle1.id).await?, circle1);
        repository.delete(&circle1).await?;
        assert!(repository.find_by_id(&circle1.id).await.is_err());
        Ok(())
    }

    fn build_circle() -> anyhow::Result<Circle> {
        Circle::new(
            "Music club".to_string(),
            Member::new("member_name1".to_string(), 21, Grade::Third, Major::Art),
            3,
        )
    }
}
