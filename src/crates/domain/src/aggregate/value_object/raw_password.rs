use std::fmt;

/// A plaintext password that has passed the domain's password policy.
///
/// Transient by design: it exists only between the request boundary and the hasher, and is
/// never stored on an aggregate or persisted.
#[derive(Clone, PartialEq, Eq)]
pub struct RawPassword(String);

const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 128;

impl RawPassword {
    pub fn new(raw: impl Into<String>) -> Result<Self, anyhow::Error> {
        let raw = raw.into();

        if raw.chars().count() < MIN_LENGTH {
            return Err(anyhow::Error::msg(format!(
                "Password must be at least {MIN_LENGTH} characters"
            )));
        }
        // Argon2 itself has no practical limit, but an unbounded password is a cheap way to
        // make the server burn CPU on hashing.
        if raw.chars().count() > MAX_LENGTH {
            return Err(anyhow::Error::msg(format!(
                "Password must be at most {MAX_LENGTH} characters"
            )));
        }

        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Redacted so that a stray `{:?}` in a log line cannot leak the password.
impl fmt::Debug for RawPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RawPassword(********)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepts_a_long_enough_password() -> anyhow::Result<()> {
        let password = RawPassword::new("correct horse battery staple")?;
        assert_eq!(password.as_str(), "correct horse battery staple");
        Ok(())
    }

    #[test]
    fn test_rejects_a_short_password() {
        assert!(RawPassword::new("short").is_err());
    }

    #[test]
    fn test_rejects_an_overlong_password() {
        assert!(RawPassword::new("a".repeat(MAX_LENGTH + 1)).is_err());
    }

    #[test]
    fn test_debug_is_redacted() -> anyhow::Result<()> {
        let password = RawPassword::new("correct horse battery staple")?;
        assert_eq!(format!("{:?}", password), "RawPassword(********)");
        Ok(())
    }
}
