use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(usize);

impl CircleId {
    pub fn new(id: usize) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let circle_id = CircleId::new(1);
        assert_eq!(circle_id.to_string(), "1");
        assert_eq!(usize::from(circle_id), 1);
    }
}
