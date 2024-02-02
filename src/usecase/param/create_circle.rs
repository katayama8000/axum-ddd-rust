use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCircleParam {
    pub id: usize,
    pub circle_name: String,
    pub owner_name: String,
    pub capacity: usize,
}
