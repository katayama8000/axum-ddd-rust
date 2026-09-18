use std::fmt;

/// An already-hashed password. The domain never holds a raw password: hashing happens
/// behind `PasswordHasherInterface`, and only the result travels through the aggregate.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hashed: impl Into<String>) -> Result<Self, anyhow::Error> {
        let hashed = hashed.into();
        if hashed.is_empty() {
            return Err(anyhow::Error::msg("Password hash must not be empty"));
        }
        Ok(Self(hashed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Redacted so that a stray `{:?}` in a log line cannot leak the hash.
impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PasswordHash(********)")
    }
}

impl From<PasswordHash> for String {
    fn from(password_hash: PasswordHash) -> Self {
        password_hash.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holds_the_hash() -> anyhow::Result<()> {
        let hash = PasswordHash::new("$argon2id$v=19$m=19456,t=2,p=1$abc$def")?;
        assert_eq!(hash.as_str(), "$argon2id$v=19$m=19456,t=2,p=1$abc$def");
        Ok(())
    }

    #[test]
    fn test_rejects_empty() {
        assert!(PasswordHash::new("").is_err());
    }

    #[test]
    fn test_debug_is_redacted() -> anyhow::Result<()> {
        let hash = PasswordHash::new("super-secret-hash")?;
        assert_eq!(format!("{:?}", hash), "PasswordHash(********)");
        Ok(())
    }
}
