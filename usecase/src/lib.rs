use {{ crate_name }}_core::repos::ReposError;
use thiserror::Error;

pub mod manager;
pub mod user_usecase;

#[cfg(test)]
mod test_helper;

#[derive(Error, Debug)]
pub enum UsecaseError {
    #[error("repos error: #{0}")]
    ReposError(#[from] ReposError),
    #[error("rdb error: #{0}")]
    RDBError(#[from] {{ crate_name }}_core::rdb::RDBError),
}

pub type UsecaseResult<T> = Result<T, UsecaseError>;
