use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FetchCircleParam {
    pub id: usize,
}
