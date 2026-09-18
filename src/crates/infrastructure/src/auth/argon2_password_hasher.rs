use argon2::{
    password_hash::{
        try_generate_salt, Error as PasswordHashError, PasswordHasher, PasswordVerifier,
    },
    Argon2,
};
use domain::{
    aggregate::value_object::{password_hash::PasswordHash, raw_password::RawPassword},
    interface::password_hasher_interface::PasswordHasherInterface,
};

/// Argon2id with the crate defaults, which follow the OWASP recommendation.
#[derive(Clone, Debug, Default)]
pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasherInterface for Argon2PasswordHasher {
    fn hash(&self, raw_password: &RawPassword) -> Result<PasswordHash, anyhow::Error> {
        let salt = try_generate_salt()
            .map_err(|e| anyhow::Error::msg(format!("Failed to generate a salt: {e}")))?;

        let hashed = Argon2::default()
            .hash_password_with_salt(raw_password.as_str().as_bytes(), &salt)
            .map_err(|e| anyhow::Error::msg(format!("Failed to hash password: {e}")))?
            .to_string();

        PasswordHash::new(hashed)
    }

    fn verify(
        &self,
        raw_password: &RawPassword,
        password_hash: &PasswordHash,
    ) -> Result<bool, anyhow::Error> {
        let result = PasswordVerifier::<str>::verify_password(
            &Argon2::default(),
            raw_password.as_str().as_bytes(),
            password_hash.as_str(),
        );

        match result {
            Ok(()) => Ok(true),
            Err(PasswordHashError::PasswordInvalid) => Ok(false),
            // Anything else means we could not check at all — a corrupt row, say — which is
            // a different thing from "the password was wrong".
            Err(e) => Err(anyhow::Error::msg(format!(
                "Failed to verify password: {e}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_then_verify() -> anyhow::Result<()> {
        let hasher = Argon2PasswordHasher::new();
        let password = RawPassword::new("correct horse battery staple")?;

        let hash = hasher.hash(&password)?;

        assert!(hasher.verify(&password, &hash)?);
        Ok(())
    }

    #[test]
    fn test_verify_rejects_a_wrong_password() -> anyhow::Result<()> {
        let hasher = Argon2PasswordHasher::new();
        let hash = hasher.hash(&RawPassword::new("correct horse battery staple")?)?;

        let wrong = RawPassword::new("incorrect horse battery staple")?;

        assert!(!hasher.verify(&wrong, &hash)?);
        Ok(())
    }

    #[test]
    fn test_hashing_the_same_password_twice_gives_different_hashes() -> anyhow::Result<()> {
        let hasher = Argon2PasswordHasher::new();
        let password = RawPassword::new("correct horse battery staple")?;

        // Different salts, so identical passwords are not detectable from the stored hashes.
        assert_ne!(hasher.hash(&password)?, hasher.hash(&password)?);
        Ok(())
    }

    #[test]
    fn test_verify_errors_on_a_malformed_hash() -> anyhow::Result<()> {
        let hasher = Argon2PasswordHasher::new();
        let password = RawPassword::new("correct horse battery staple")?;

        let result = hasher.verify(&password, &PasswordHash::new("not-a-hash")?);

        assert!(result.is_err());
        Ok(())
    }
}
