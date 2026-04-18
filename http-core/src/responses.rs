pub mod user_response;

use {{ crate_name }}_domain::repos;
use {{ crate_name }}_usecase::UsecaseError;

#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    #[error("RDB error: {0}")]
    RDB(#[from] {{ crate_name }}_domain::rdb::RDBError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Repos error: {0}")]
    Repos(#[from] repos::ReposError),
    #[error("Usecase error: {0}")]
    Usecase(#[from] UsecaseError),
}

pub type ResponseResult<T> = Result<T, ResponseError>;
