use anyhow::Result;
use serde::Deserialize;

use domain::{
    aggregate::{
        user::User,
        value_object::{email::Email, raw_password::RawPassword},
    },
    interface::{
        access_token_issuer_interface::AccessTokenIssuerInterface,
        password_hasher_interface::PasswordHasherInterface,
        user_repository_interface::UserRepositoryInterface,
    },
};

#[derive(Debug, Deserialize)]
pub struct SignUpInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SignUpOutput {
    pub user_id: String,
    pub access_token: String,
    pub expires_in: u64,
}

/// Returned when the email is already taken, so the handler can answer 409 instead of 500.
#[derive(Debug, thiserror::Error)]
pub enum SignUpError {
    #[error("Email is already registered")]
    EmailAlreadyTaken,
    #[error("{0}")]
    InvalidInput(#[source] anyhow::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct SignUpUsecase<R, H, T>
where
    R: UserRepositoryInterface,
    H: PasswordHasherInterface,
    T: AccessTokenIssuerInterface,
{
    user_repository: R,
    password_hasher: H,
    access_token_issuer: T,
}

impl<R, H, T> SignUpUsecase<R, H, T>
where
    R: UserRepositoryInterface,
    H: PasswordHasherInterface,
    T: AccessTokenIssuerInterface,
{
    pub fn new(user_repository: R, password_hasher: H, access_token_issuer: T) -> Self {
        SignUpUsecase {
            user_repository,
            password_hasher,
            access_token_issuer,
        }
    }

    pub async fn execute(
        &mut self,
        sign_up_input: SignUpInput,
    ) -> Result<SignUpOutput, SignUpError> {
        let email = Email::new(sign_up_input.email).map_err(SignUpError::InvalidInput)?;
        let raw_password =
            RawPassword::new(sign_up_input.password).map_err(SignUpError::InvalidInput)?;

        // A unique index on users.email is what actually guarantees uniqueness; this check
        // exists to turn the common case into a clear error instead of a constraint violation.
        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(SignUpError::EmailAlreadyTaken);
        }

        let password_hash = self.password_hasher.hash(&raw_password)?;
        let user = User::new(email, password_hash);

        self.user_repository.create(&user).await?;

        let access_token = self.access_token_issuer.issue(&user.id)?;

        Ok(SignUpOutput {
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
        aggregate::value_object::{password_hash::PasswordHash, user_id::UserId},
        interface::{
            access_token_issuer_interface::{AccessToken, MockAccessTokenIssuerInterface},
            password_hasher_interface::MockPasswordHasherInterface,
            user_repository_interface::MockUserRepositoryInterface,
        },
    };

    fn access_token() -> AccessToken {
        AccessToken {
            value: "token".to_string(),
            expires_in: 3600,
        }
    }

    #[tokio::test]
    async fn test_sign_up() -> anyhow::Result<()> {
        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository
            .expect_find_by_email()
            .times(1)
            .return_once(|_| Ok(None));
        user_repository
            .expect_create()
            .times(1)
            .return_once(|_| Ok(()));

        let mut password_hasher = MockPasswordHasherInterface::new();
        password_hasher
            .expect_hash()
            .times(1)
            .return_once(|_| PasswordHash::new("hashed"));

        let mut access_token_issuer = MockAccessTokenIssuerInterface::new();
        access_token_issuer
            .expect_issue()
            .times(1)
            .return_once(|_| Ok(access_token()));

        let mut usecase = SignUpUsecase::new(user_repository, password_hasher, access_token_issuer);
        let output = usecase
            .execute(SignUpInput {
                email: "user@example.com".to_string(),
                password: "correct horse battery staple".to_string(),
            })
            .await?;

        assert_eq!(output.access_token, "token");
        assert_eq!(output.expires_in, 3600);
        assert_eq!(output.user_id.len(), 36);
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_up_rejects_a_taken_email() {
        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository
            .expect_find_by_email()
            .times(1)
            .return_once(|email| {
                Ok(Some(User::reconstruct(
                    UserId::gen(),
                    email.clone(),
                    PasswordHash::new("hashed").unwrap(),
                )))
            });
        // Neither hashing nor persistence should be reached.
        user_repository.expect_create().never();

        let mut password_hasher = MockPasswordHasherInterface::new();
        password_hasher.expect_hash().never();

        let mut access_token_issuer = MockAccessTokenIssuerInterface::new();
        access_token_issuer.expect_issue().never();

        let mut usecase = SignUpUsecase::new(user_repository, password_hasher, access_token_issuer);
        let result = usecase
            .execute(SignUpInput {
                email: "user@example.com".to_string(),
                password: "correct horse battery staple".to_string(),
            })
            .await;

        assert!(matches!(result, Err(SignUpError::EmailAlreadyTaken)));
    }

    #[tokio::test]
    async fn test_sign_up_rejects_a_short_password() {
        let mut user_repository = MockUserRepositoryInterface::new();
        // Input is validated before anything touches the database.
        user_repository.expect_find_by_email().never();
        user_repository.expect_create().never();

        let mut usecase = SignUpUsecase::new(
            user_repository,
            MockPasswordHasherInterface::new(),
            MockAccessTokenIssuerInterface::new(),
        );
        let result = usecase
            .execute(SignUpInput {
                email: "user@example.com".to_string(),
                password: "short".to_string(),
            })
            .await;

        assert!(matches!(result, Err(SignUpError::InvalidInput(_))));
    }

    #[tokio::test]
    async fn test_sign_up_rejects_an_invalid_email() {
        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository.expect_find_by_email().never();
        user_repository.expect_create().never();

        let mut usecase = SignUpUsecase::new(
            user_repository,
            MockPasswordHasherInterface::new(),
            MockAccessTokenIssuerInterface::new(),
        );
        let result = usecase
            .execute(SignUpInput {
                email: "not-an-email".to_string(),
                password: "correct horse battery staple".to_string(),
            })
            .await;

        assert!(matches!(result, Err(SignUpError::InvalidInput(_))));
    }
}
