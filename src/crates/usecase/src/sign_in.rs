use anyhow::Result;
use serde::Deserialize;

use domain::{
    aggregate::value_object::{email::Email, raw_password::RawPassword},
    interface::{
        access_token_issuer_interface::AccessTokenIssuerInterface,
        password_hasher_interface::PasswordHasherInterface,
        user_repository_interface::UserRepositoryInterface,
    },
};

#[derive(Debug, Deserialize)]
pub struct SignInInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SignInOutput {
    pub user_id: String,
    pub access_token: String,
    pub expires_in: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SignInError {
    /// Deliberately one variant for "no such user" and "wrong password". Telling them apart
    /// would let an attacker enumerate which email addresses are registered.
    #[error("Email or password is incorrect")]
    InvalidCredentials,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct SignInUsecase<R, H, T>
where
    R: UserRepositoryInterface,
    H: PasswordHasherInterface,
    T: AccessTokenIssuerInterface,
{
    user_repository: R,
    password_hasher: H,
    access_token_issuer: T,
}

impl<R, H, T> SignInUsecase<R, H, T>
where
    R: UserRepositoryInterface,
    H: PasswordHasherInterface,
    T: AccessTokenIssuerInterface,
{
    pub fn new(user_repository: R, password_hasher: H, access_token_issuer: T) -> Self {
        SignInUsecase {
            user_repository,
            password_hasher,
            access_token_issuer,
        }
    }

    pub async fn execute(&self, sign_in_input: SignInInput) -> Result<SignInOutput, SignInError> {
        // A malformed email cannot match a stored one, so answer as if the lookup failed
        // rather than leaking that validation is what rejected it.
        let email = Email::new(sign_in_input.email).map_err(|_| SignInError::InvalidCredentials)?;
        let raw_password = RawPassword::new(sign_in_input.password)
            .map_err(|_| SignInError::InvalidCredentials)?;

        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(SignInError::InvalidCredentials)?;

        if !self
            .password_hasher
            .verify(&raw_password, &user.password_hash)?
        {
            return Err(SignInError::InvalidCredentials);
        }

        let access_token = self.access_token_issuer.issue(&user.id)?;

        Ok(SignInOutput {
            user_id: String::from(user.id),
            access_token: access_token.value,
            expires_in: access_token.expires_in,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{
        aggregate::{
            user::User,
            value_object::{password_hash::PasswordHash, user_id::UserId},
        },
        interface::{
            access_token_issuer_interface::{AccessToken, MockAccessTokenIssuerInterface},
            password_hasher_interface::MockPasswordHasherInterface,
            user_repository_interface::MockUserRepositoryInterface,
        },
    };

    fn stored_user() -> User {
        User::reconstruct(
            UserId::gen(),
            Email::new("user@example.com").unwrap(),
            PasswordHash::new("hashed").unwrap(),
        )
    }

    #[tokio::test]
    async fn test_sign_in() -> anyhow::Result<()> {
        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository
            .expect_find_by_email()
            .times(1)
            .return_once(|_| Ok(Some(stored_user())));

        let mut password_hasher = MockPasswordHasherInterface::new();
        password_hasher
            .expect_verify()
            .times(1)
            .return_once(|_, _| Ok(true));

        let mut access_token_issuer = MockAccessTokenIssuerInterface::new();
        access_token_issuer
            .expect_issue()
            .times(1)
            .return_once(|_| {
                Ok(AccessToken {
                    value: "token".to_string(),
                    expires_in: 3600,
                })
            });

        let usecase = SignInUsecase::new(user_repository, password_hasher, access_token_issuer);
        let output = usecase
            .execute(SignInInput {
                email: "user@example.com".to_string(),
                password: "correct horse battery staple".to_string(),
            })
            .await?;

        assert_eq!(output.access_token, "token");
        assert_eq!(output.expires_in, 3600);
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_in_rejects_a_wrong_password() {
        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository
            .expect_find_by_email()
            .times(1)
            .return_once(|_| Ok(Some(stored_user())));

        let mut password_hasher = MockPasswordHasherInterface::new();
        password_hasher
            .expect_verify()
            .times(1)
            .return_once(|_, _| Ok(false));

        let mut access_token_issuer = MockAccessTokenIssuerInterface::new();
        access_token_issuer.expect_issue().never();

        let usecase = SignInUsecase::new(user_repository, password_hasher, access_token_issuer);
        let result = usecase
            .execute(SignInInput {
                email: "user@example.com".to_string(),
                password: "correct horse battery staple".to_string(),
            })
            .await;

        assert!(matches!(result, Err(SignInError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_sign_in_rejects_an_unknown_email() {
        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository
            .expect_find_by_email()
            .times(1)
            .return_once(|_| Ok(None));

        let mut password_hasher = MockPasswordHasherInterface::new();
        password_hasher.expect_verify().never();

        let mut access_token_issuer = MockAccessTokenIssuerInterface::new();
        access_token_issuer.expect_issue().never();

        let usecase = SignInUsecase::new(user_repository, password_hasher, access_token_issuer);
        let result = usecase
            .execute(SignInInput {
                email: "nobody@example.com".to_string(),
                password: "correct horse battery staple".to_string(),
            })
            .await;

        // Same error as a wrong password, so the response cannot be used to enumerate users.
        assert!(matches!(result, Err(SignInError::InvalidCredentials)));
    }
}
