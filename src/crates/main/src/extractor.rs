use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use domain::interface::access_token_issuer_interface::AccessTokenIssuerInterface;

use crate::{app::AppState, error::AppError};

/// A caller proven to hold a valid access token.
///
/// Taking this as a handler argument is what makes a route protected — there is no way to write
/// the handler and forget the check, because the id can only be obtained through this extractor.
#[derive(Debug, Clone)]
pub(crate) struct AuthUser {
    pub(crate) user_id: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AppError::Unauthorized)?
            .to_str()
            .map_err(|_| AppError::Unauthorized)?;

        let token = parse_bearer(header).ok_or(AppError::Unauthorized)?;

        let user_id = state
            .access_token_issuer
            .verify(token)
            // Why the token failed (expired, bad signature, malformed) is useful in a log but
            // not in the response.
            .map_err(|e| {
                tracing::debug!("rejected access token: {}", e);
                AppError::Unauthorized
            })?;

        Ok(AuthUser {
            user_id: user_id.to_string(),
        })
    }
}

/// `Bearer <token>`, with the scheme compared case-insensitively as RFC 7235 requires.
fn parse_bearer(header: &str) -> Option<&str> {
    let (scheme, token) = header.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("Bearer") {
        return None;
    }
    let token = token.trim();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bearer() {
        assert_eq!(parse_bearer("Bearer abc.def.ghi"), Some("abc.def.ghi"));
        // The scheme is case-insensitive.
        assert_eq!(parse_bearer("bearer abc.def.ghi"), Some("abc.def.ghi"));
        assert_eq!(parse_bearer("BEARER abc.def.ghi"), Some("abc.def.ghi"));
    }

    #[test]
    fn test_parse_bearer_rejects_other_shapes() {
        assert_eq!(parse_bearer(""), None);
        assert_eq!(parse_bearer("abc.def.ghi"), None);
        assert_eq!(parse_bearer("Basic dXNlcjpwYXNz"), None);
        assert_eq!(parse_bearer("Bearer "), None);
        assert_eq!(parse_bearer("Bearer    "), None);
    }
}
