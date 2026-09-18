use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use domain::{
    aggregate::value_object::user_id::UserId,
    interface::access_token_issuer_interface::{AccessToken, AccessTokenIssuerInterface},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Registered JWT claims. `sub` carries the user id; there is deliberately nothing else in
/// here, because anyone holding the token can read it — it is signed, not encrypted.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

#[derive(Clone)]
pub struct JwtAccessTokenIssuer {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    ttl_seconds: u64,
}

/// Hand-written so that the keys never reach a log line.
impl std::fmt::Debug for JwtAccessTokenIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtAccessTokenIssuer")
            .field("ttl_seconds", &self.ttl_seconds)
            .finish_non_exhaustive()
    }
}

impl JwtAccessTokenIssuer {
    pub fn new(secret: &str, ttl_seconds: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            ttl_seconds,
        }
    }

    fn now() -> Result<u64, anyhow::Error> {
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow::Error::msg(format!("System clock is before the epoch: {e}")))?
            .as_secs())
    }
}

impl AccessTokenIssuerInterface for JwtAccessTokenIssuer {
    fn issue(&self, user_id: &UserId) -> Result<AccessToken, anyhow::Error> {
        let issued_at = Self::now()?;
        let claims = Claims {
            sub: user_id.to_string(),
            iat: issued_at,
            exp: issued_at + self.ttl_seconds,
        };

        let value = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| anyhow::Error::msg(format!("Failed to sign access token: {e}")))?;

        Ok(AccessToken {
            value,
            expires_in: self.ttl_seconds,
        })
    }

    fn verify(&self, token: &str) -> Result<UserId, anyhow::Error> {
        // Pinning the algorithm matters: without it a token could ask to be verified with
        // "none", or with a different algorithm than the one we issue.
        let validation = Validation::new(Algorithm::HS256);

        let decoded = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| anyhow::Error::msg(format!("Invalid access token: {e}")))?;

        UserId::from_str(&decoded.claims.sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "test-secret-that-is-long-enough";

    #[test]
    fn test_issue_then_verify() -> anyhow::Result<()> {
        let issuer = JwtAccessTokenIssuer::new(SECRET, 3600);
        let user_id = UserId::gen();

        let token = issuer.issue(&user_id)?;

        assert_eq!(token.expires_in, 3600);
        assert_eq!(issuer.verify(&token.value)?, user_id);
        Ok(())
    }

    #[test]
    fn test_verify_rejects_another_secret() -> anyhow::Result<()> {
        let token = JwtAccessTokenIssuer::new(SECRET, 3600).issue(&UserId::gen())?;

        let other = JwtAccessTokenIssuer::new("a-completely-different-secret", 3600);

        assert!(other.verify(&token.value).is_err());
        Ok(())
    }

    #[test]
    fn test_verify_rejects_an_expired_token() -> anyhow::Result<()> {
        let issuer = JwtAccessTokenIssuer::new(SECRET, 3600);
        let user_id = UserId::gen();

        // Signed with the real key but already expired. jsonwebtoken allows 60s of clock skew
        // by default, so `exp` has to be comfortably in the past.
        let issued_at = JwtAccessTokenIssuer::now()? - 7200;
        let expired = encode(
            &Header::new(Algorithm::HS256),
            &Claims {
                sub: user_id.to_string(),
                iat: issued_at,
                exp: issued_at + 3600,
            },
            &issuer.encoding_key,
        )?;

        assert!(issuer.verify(&expired).is_err());
        Ok(())
    }

    #[test]
    fn test_verify_rejects_garbage() {
        let issuer = JwtAccessTokenIssuer::new(SECRET, 3600);

        assert!(issuer.verify("not.a.jwt").is_err());
        assert!(issuer.verify("").is_err());
    }

    #[test]
    fn test_debug_does_not_leak_the_key() {
        let issuer = JwtAccessTokenIssuer::new(SECRET, 3600);

        assert!(!format!("{:?}", issuer).contains(SECRET));
    }
}
