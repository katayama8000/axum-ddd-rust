use super::value_object::{email::Email, password_hash::PasswordHash, user_id::UserId};

/// The authentication principal. Kept separate from `Member`, which is a child entity of the
/// `Circle` aggregate and carries no credentials.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
}

impl User {
    pub fn new(email: Email, password_hash: PasswordHash) -> Self {
        User {
            id: UserId::gen(),
            email,
            password_hash,
        }
    }

    pub fn reconstruct(id: UserId, email: Email, password_hash: PasswordHash) -> Self {
        User {
            id,
            email,
            password_hash,
        }
    }

    pub fn change_password(&mut self, password_hash: PasswordHash) {
        self.password_hash = password_hash;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn password_hash(value: &str) -> PasswordHash {
        PasswordHash::new(value).expect("hash should not be empty")
    }

    #[test]
    fn test_user_new() -> anyhow::Result<()> {
        let email = Email::new("user@example.com")?;
        let user = User::new(email.clone(), password_hash("hashed"));
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash("hashed"));
        assert_eq!(user.id.to_string().len(), 36);
        Ok(())
    }

    #[test]
    fn test_user_reconstruct() -> anyhow::Result<()> {
        let user_id = UserId::from_str("0123456789abcdef0123456789abcdef0123")?;
        let email = Email::new("user@example.com")?;
        let user = User::reconstruct(user_id.clone(), email.clone(), password_hash("hashed"));
        assert_eq!(user.id, user_id);
        assert_eq!(user.email, email);
        Ok(())
    }

    #[test]
    fn test_change_password() -> anyhow::Result<()> {
        let mut user = User::new(Email::new("user@example.com")?, password_hash("old"));
        user.change_password(password_hash("new"));
        assert_eq!(user.password_hash, password_hash("new"));
        Ok(())
    }
}
