use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCircleInputParam {
    pub id: usize,
    pub circle_name: String,
    pub owner_name: String,
    pub capacity: usize,
}
