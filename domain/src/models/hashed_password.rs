use fake::Dummy;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordHashError {
    #[error("hash failed: {0}")]
    HashFailed(#[from] bcrypt::BcryptError),
}

#[derive(Debug, Clone, PartialEq, Dummy)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_plain(password: &str) -> Result<Self, PasswordHashError> {
        const BCRYPT_DEFAULT_COST: u32 = 4;
        Ok(Self(bcrypt::hash(password, BCRYPT_DEFAULT_COST)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
