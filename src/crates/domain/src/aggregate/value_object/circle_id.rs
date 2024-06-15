use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use rand::distributions::{Alphanumeric, DistString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(String);

impl CircleId {
    pub fn gen() -> Self {
        let mut rng = rand::thread_rng();
        Self(Alphanumeric.sample_string(&mut rng, 36))
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

impl FromStr for CircleId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<i16> for CircleId {
    fn from(id: i16) -> Self {
        Self(id.to_string())
    }
}

impl From<CircleId> for String {
    fn from(circle_id: CircleId) -> Self {
        circle_id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let circle_id = CircleId::gen();
        assert_eq!(circle_id.to_string().len(), 36);

        let str = "0123456789abcdef0123456789abcdef";
        let circle_id = CircleId::from_str(str)?;
        assert_eq!(circle_id.to_string(), str);
        Ok(())
    }
}
