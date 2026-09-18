use std::str::FromStr;

use domain::aggregate::{
    user::User,
    value_object::{email::Email, password_hash::PasswordHash, user_id::UserId},
};

#[derive(Debug)]
pub struct UserData {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}

impl std::convert::TryFrom<UserData> for User {
    type Error = anyhow::Error;

    fn try_from(data: UserData) -> Result<Self, Self::Error> {
        let user_id = UserId::from_str(data.id.as_str())?;
        let email = Email::new(data.email)?;
        let password_hash = PasswordHash::new(data.password_hash)?;

        Ok(User::reconstruct(user_id, email, password_hash))
    }
}

impl std::convert::From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            id: String::from(user.id),
            email: user.email.to_string(),
            password_hash: String::from(user.password_hash),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() -> anyhow::Result<()> {
        let user = User::new(
            Email::new("user@example.com")?,
            PasswordHash::new("hashed")?,
        );
        let data = UserData::from(user.clone());
        let restored = User::try_from(data)?;

        assert_eq!(restored, user);
        Ok(())
    }

    #[test]
    fn test_rejects_a_row_with_a_broken_email() {
        let data = UserData {
            id: "0123456789abcdef0123456789abcdef0123".to_string(),
            email: "not-an-email".to_string(),
            password_hash: "hashed".to_string(),
        };

        assert!(User::try_from(data).is_err());
    }
}
