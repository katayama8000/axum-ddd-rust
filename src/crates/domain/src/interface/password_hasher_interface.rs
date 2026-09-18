use crate::aggregate::value_object::{password_hash::PasswordHash, raw_password::RawPassword};
use anyhow::Error;

/// Port for the password hashing algorithm. The domain states *that* passwords are hashed and
/// verified; which algorithm does it is an infrastructure decision.
#[mockall::automock]
pub trait PasswordHasherInterface {
    fn hash(&self, raw_password: &RawPassword) -> Result<PasswordHash, Error>;

    /// `Ok(false)` means the password did not match. `Err` means verification could not be
    /// performed at all, e.g. a stored hash that is not in the expected format.
    fn verify(
        &self,
        raw_password: &RawPassword,
        password_hash: &PasswordHash,
    ) -> Result<bool, Error>;
}
