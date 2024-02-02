use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateCircleParam {
    pub id: usize,
    pub circle_name: Option<String>,
    pub capacity: Option<usize>,
}
