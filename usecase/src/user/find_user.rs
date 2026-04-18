use crate::UsecaseResult;
use async_trait::async_trait;
use {{ crate_name }}_domain::models::user::{User, UserId};
use {{ crate_name }}_domain::rdb::HaveRDBPool;
use {{ crate_name }}_domain::repos::user_repository::{HaveUserRepository, UserRepository};

#[async_trait]
pub trait FindUserUsecase {
    async fn find_user(&self, id: &UserId) -> UsecaseResult<Option<User>>;
}

pub trait FindUserUsecaseImpl: Sync + HaveRDBPool + HaveUserRepository {}

#[async_trait]
impl<T: FindUserUsecaseImpl> FindUserUsecase for T {
    async fn find_user(&self, id: &UserId) -> UsecaseResult<Option<User>> {
        let mut conn = self.get_rdb_pool().acquire().await?;
        let user = self.user_repo().find(&mut conn, id).await?;
        Ok(user)
    }
}
