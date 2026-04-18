use {{ crate_name }}_domain::models::PasswordHashError;
use {{ crate_name }}_domain::repos::ReposError;
use thiserror::Error;

pub mod manager;
pub mod user;

#[cfg(test)]
mod test_helper;

#[derive(Error, Debug)]
pub enum UsecaseError {
    #[error("repos error: #{0}")]
    ReposError(#[from] ReposError),
    #[error("rdb error: #{0}")]
    RDBError(#[from] {{ crate_name }}_domain::rdb::RDBError),
    #[error("password hash error: #{0}")]
    PasswordHash(#[from] PasswordHashError),
}

pub type UsecaseResult<T> = Result<T, UsecaseError>;
