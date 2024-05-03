use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct MemberId(i32);

impl MemberId {
    pub fn gen() -> Self {
        Self(rand::random::<i32>())
    }
}

impl Hash for MemberId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for MemberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<i32> for MemberId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl std::convert::From<MemberId> for i32 {
    fn from(member_id: MemberId) -> Self {
        member_id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let member_id = MemberId::from(1);
        assert_eq!(member_id.to_string(), "1");
        assert_eq!(i32::from(member_id), 1);
    }
}
