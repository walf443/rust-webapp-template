use crate::repos::ReposError;
use thiserror::Error;

pub mod manager;
pub mod user_service;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("repos error: #{0}")]
    ReposError(#[from] ReposError),
    #[error("rdb error: #{0}")]
    RDBError(#[from] crate::rdb::RDBError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;
