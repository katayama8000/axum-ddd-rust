use std::fmt;
use std::hash::{Hash, Hasher};

use rand::distributions::{Alphanumeric, DistString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberId(String);

impl MemberId {
    pub fn gen() -> Self {
        let mut rng = rand::thread_rng();
        Self(Alphanumeric.sample_string(&mut rng, 36))
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

impl std::str::FromStr for MemberId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl std::convert::From<i16> for MemberId {
    fn from(id: i16) -> Self {
        Self(id.to_string())
    }
}

impl std::convert::From<MemberId> for String {
    fn from(member_id: MemberId) -> Self {
        member_id.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let member_id = MemberId::gen();
        assert_eq!(member_id.to_string().len(), 36);

        let str = "0123456789abcdef0123456789abcdef";
        let member_id = MemberId::from_str(str)?;
        assert_eq!(member_id.to_string(), str);
        Ok(())
    }
}
