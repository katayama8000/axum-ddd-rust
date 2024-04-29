use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(usize);

impl CircleId {
    pub fn gen() -> Self {
        Self(rand::random::<usize>())
    }
}

impl std::convert::From<usize> for CircleId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl Hash for CircleId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for CircleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<CircleId> for usize {
    fn from(circle_id: CircleId) -> usize {
        circle_id.0
    }
}

// インフラ層に書くべき
impl std::convert::From<CircleId> for Vec<u8> {
    fn from(id: CircleId) -> Vec<u8> {
        id.0.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let circle_id = CircleId::from(1);
        assert_eq!(circle_id.to_string(), "1");
        assert_eq!(usize::from(circle_id), 1);
    }
}
