use crate::aggregate::{
    user::User,
    value_object::{email::Email, user_id::UserId},
};
use anyhow::Error;

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserRepositoryInterface {
    /// `Ok(None)` means "no such user", which is a normal outcome during sign-in.
    /// Reserve `Err` for failures the caller cannot act on, such as a dead connection.
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, Error>;
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, Error>;
    async fn create(&self, user: &User) -> Result<(), Error>;
}
