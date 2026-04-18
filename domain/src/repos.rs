use thiserror::Error;

pub mod manager;
pub mod user_repository;

#[derive(Debug, Error)]
pub enum ReposError {
    #[error("RDB error: {0}")]
    RDB(#[from] crate::rdb::RDBError),
    #[error("test error")]
    TestError,
}

pub type Result<T> = std::result::Result<T, ReposError>;
