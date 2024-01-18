use std::fmt;
use std::hash::{Hash, Hasher};

// CircleのID (サークルのID)
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
