use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use rand::distr::Alphanumeric;
use rand::RngExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn gen() -> Self {
        let mut rng = rand::rng();
        let chars: String = (0..36).map(|_| rng.sample(Alphanumeric) as char).collect();
        Self(chars)
    }
}

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<UserId> for String {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let user_id = UserId::gen();
        assert_eq!(user_id.to_string().len(), 36);

        let str = "0123456789abcdef0123456789abcdef";
        let user_id = UserId::from_str(str)?;
        assert_eq!(user_id.to_string(), str);
        Ok(())
    }
}
