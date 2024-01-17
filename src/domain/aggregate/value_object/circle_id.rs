use std::fmt;
use std::hash::{Hash, Hasher};

// CircleのID (サークルのID)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(pub usize);

// Value Objectの比較用の実装
impl Hash for CircleId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

// Value Objectの表示用の実装
impl fmt::Display for CircleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
