use std::fmt;
use std::str::FromStr;

/// A validated email address. Stored lowercased so that lookups are case-insensitive.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(raw: impl Into<String>) -> Result<Self, anyhow::Error> {
        let normalized = raw.into().trim().to_lowercase();

        if normalized.len() > 254 {
            return Err(anyhow::Error::msg("Email is too long"));
        }

        // Deliberately loose: exactly one '@', with a non-empty local part and a
        // domain that contains a dot. Anything stricter rejects valid addresses.
        let (local, domain) = normalized
            .split_once('@')
            .ok_or_else(|| anyhow::Error::msg("Email must contain '@'"))?;

        if local.is_empty() || domain.is_empty() {
            return Err(anyhow::Error::msg(
                "Email has an empty local part or domain",
            ));
        }
        if domain.contains('@') {
            return Err(anyhow::Error::msg("Email must contain exactly one '@'"));
        }
        if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
            return Err(anyhow::Error::msg("Email domain is invalid"));
        }
        if normalized.chars().any(char::is_whitespace) {
            return Err(anyhow::Error::msg("Email must not contain whitespace"));
        }

        Ok(Self(normalized))
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Email {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepts_a_valid_address() -> anyhow::Result<()> {
        let email = Email::new("user@example.com")?;
        assert_eq!(email.to_string(), "user@example.com");
        Ok(())
    }

    #[test]
    fn test_normalizes_case_and_surrounding_spaces() -> anyhow::Result<()> {
        let email = Email::new("  User@Example.COM  ")?;
        assert_eq!(email.to_string(), "user@example.com");
        Ok(())
    }

    #[test]
    fn test_rejects_invalid_addresses() {
        for invalid in [
            "",
            "user",
            "@example.com",
            "user@",
            "user@example",
            "user@@example.com",
            "us er@example.com",
            "user@.com",
        ] {
            assert!(Email::new(invalid).is_err(), "{invalid} should be rejected");
        }
    }
}
