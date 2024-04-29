use crate::domain::aggregate::circle::Circle;

use super::member_data::MemberData;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CircleData {
    pub id: Vec<u8>, // BINARY(16)に対応するバイト配列型に修正
    pub name: String,
    pub owner_id: Vec<u8>, // BINARY(16)に対応するバイト配列型に修正
    pub capacity: usize,
    pub members: Vec<MemberData>,
}

impl std::convert::TryFrom<CircleData> for Circle {
    type Error = anyhow::Error;

    fn try_from(data: CircleData) -> Result<Self, Self::Error> {
        todo!()
    }
}
