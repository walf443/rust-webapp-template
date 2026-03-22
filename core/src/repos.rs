use bcrypt::BcryptError;
use thiserror::Error;

pub mod manager;
pub mod user_repository;

#[derive(Debug, Error)]
pub enum ReposError {
    #[error("RDB error: {0}")]
    RDB(#[from] crate::rdb::RDBError),
    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("test error")]
    TestError,
}

pub type Result<T> = std::result::Result<T, ReposError>;
