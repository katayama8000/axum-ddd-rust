use anyhow::Result;
use serde::Deserialize;
use std::str::FromStr;

use domain::{
    aggregate::value_object::user_id::UserId,
    interface::user_repository_interface::UserRepositoryInterface,
};

#[derive(Debug, Deserialize)]
pub struct FetchMeInput {
    pub user_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct FetchMeOutput {
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchMeError {
    /// The token was valid but the user is gone, e.g. deleted after the token was issued.
    #[error("User not found")]
    UserNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct FetchMeUsecase<R>
where
    R: UserRepositoryInterface,
{
    user_repository: R,
}

impl<R> FetchMeUsecase<R>
where
    R: UserRepositoryInterface,
{
    pub fn new(user_repository: R) -> Self {
        FetchMeUsecase { user_repository }
    }

    pub async fn execute(
        &self,
        fetch_me_input: FetchMeInput,
    ) -> Result<FetchMeOutput, FetchMeError> {
        let user_id = UserId::from_str(&fetch_me_input.user_id)?;

        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or(FetchMeError::UserNotFound)?;

        Ok(FetchMeOutput {
            user_id: String::from(user.id),
            email: user.email.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{
        aggregate::{
            user::User,
            value_object::{email::Email, password_hash::PasswordHash},
        },
        interface::user_repository_interface::MockUserRepositoryInterface,
    };

    #[tokio::test]
    async fn test_fetch_me() -> anyhow::Result<()> {
        let user = User::new(
            Email::new("user@example.com")?,
            PasswordHash::new("hashed")?,
        );
        let user_id = String::from(user.id.clone());

        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository
            .expect_find_by_id()
            .times(1)
            .return_once(move |_| Ok(Some(user)));

        let usecase = FetchMeUsecase::new(user_repository);
        let output = usecase
            .execute(FetchMeInput {
                user_id: user_id.clone(),
            })
            .await?;

        assert_eq!(output.user_id, user_id);
        assert_eq!(output.email, "user@example.com");
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_me_when_the_user_is_gone() {
        let mut user_repository = MockUserRepositoryInterface::new();
        user_repository
            .expect_find_by_id()
            .times(1)
            .return_once(|_| Ok(None));

        let usecase = FetchMeUsecase::new(user_repository);
        let result = usecase
            .execute(FetchMeInput {
                user_id: "0123456789abcdef0123456789abcdef0123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(FetchMeError::UserNotFound)));
    }
}
