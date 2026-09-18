use std::env;

const DEFAULT_ACCESS_TOKEN_TTL_SECONDS: u64 = 3600;
/// 32 bytes of entropy is the usual floor for HMAC-SHA256; a short secret is brute-forceable.
const MIN_SECRET_LENGTH: usize = 32;

#[derive(Clone, Debug)]
pub(crate) struct AuthConfig {
    pub(crate) jwt_secret: String,
    pub(crate) access_token_ttl_seconds: u64,
}

impl AuthConfig {
    pub(crate) fn from_env() -> Result<Self, anyhow::Error> {
        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| anyhow::Error::msg("JWT_SECRET must be set"))?;

        // Failing at startup beats signing production tokens with "secret".
        if jwt_secret.len() < MIN_SECRET_LENGTH {
            return Err(anyhow::Error::msg(format!(
                "JWT_SECRET must be at least {MIN_SECRET_LENGTH} characters"
            )));
        }

        let access_token_ttl_seconds = match env::var("ACCESS_TOKEN_TTL_SECONDS") {
            Err(_) => DEFAULT_ACCESS_TOKEN_TTL_SECONDS,
            Ok(raw) => raw
                .parse::<u64>()
                .map_err(|_| anyhow::Error::msg("ACCESS_TOKEN_TTL_SECONDS must be a number"))?,
        };

        if access_token_ttl_seconds == 0 {
            return Err(anyhow::Error::msg(
                "ACCESS_TOKEN_TTL_SECONDS must be greater than 0",
            ));
        }

        Ok(Self {
            jwt_secret,
            access_token_ttl_seconds,
        })
    }
}
