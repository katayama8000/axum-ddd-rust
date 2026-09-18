use crate::aggregate::value_object::user_id::UserId;
use anyhow::Error;

/// A signed, self-contained credential handed to the client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessToken {
    pub value: String,
    /// Lifetime in seconds, so a client can refresh before it expires.
    pub expires_in: u64,
}

/// Port for issuing and verifying access tokens. JWT is one implementation of this; the domain
/// and use cases only care that a token can be minted for a user and resolved back to one.
#[mockall::automock]
pub trait AccessTokenIssuerInterface {
    fn issue(&self, user_id: &UserId) -> Result<AccessToken, Error>;

    /// `Err` for anything that makes the token untrustworthy: bad signature, expired, malformed.
    fn verify(&self, token: &str) -> Result<UserId, Error>;
}
