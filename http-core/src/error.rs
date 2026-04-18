use std::borrow::Cow;

use crate::responses::ResponseError;
use axum::http::StatusCode;
use {{ crate_name }}_domain::models::PasswordHashError;
use {{ crate_name }}_domain::repos::ReposError;
use {{ crate_name }}_usecase::UsecaseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("DB error: {0}")]
    RDB(#[from] {{ crate_name }}_domain::rdb::RDBError),
    #[error("Repos error: {0}")]
    ReposError(#[from] ReposError),
    #[error("response error: {0}")]
    ResponseError(#[from] ResponseError),
    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("async-session error: {0}")]
    AsyncSession(#[from] async_session::Error),
    #[error("{0}")]
    BadRequest(Cow<'static, str>),
    #[error("session error")]
    SessionError,
    #[error("unauthorized: {0}")]
    Unauthorized(Cow<'static, str>),
    #[error("forbidden: {0}")]
    Forbidden(Cow<'static, str>),
    #[error("not found: {0}")]
    NotFound(Cow<'static, str>),
    #[error("{0}")]
    InternalServerError(String),
}

impl From<UsecaseError> for Error {
    fn from(err: UsecaseError) -> Self {
        match err {
            UsecaseError::ReposError(e) => Self::ReposError(e),
            UsecaseError::RDBError(e) => Self::RDB(e),
            UsecaseError::PasswordHash(PasswordHashError::HashFailed(e)) => Self::Bcrypt(e),
        }
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Debug, serde::Serialize)]
        struct ErrorResponse {
            error: String,
        }

        let status = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) | Self::SessionError => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Io(_)
            | Self::RDB(_)
            | Self::ReposError(_)
            | Self::ResponseError(_)
            | Self::Bcrypt(_)
            | Self::AsyncSession(_)
            | Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        tracing::error!("{}", self);
        (
            status,
            axum::Json(ErrorResponse {
                error: format!("{}", self),
            }),
        )
            .into_response()
    }
}
