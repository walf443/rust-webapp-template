use crate::UsecaseResult;
use async_trait::async_trait;
use {{ crate_name }}_domain::models::user::User;
use {{ crate_name }}_domain::rdb::HaveRDBPool;
use {{ crate_name }}_domain::repos::user_repository::{HaveUserRepository, UserRepository};

#[async_trait]
pub trait FindUserByNameUsecase {
    async fn find_user_by_name(&self, name: &str) -> UsecaseResult<Option<User>>;
}

pub trait FindUserByNameUsecaseImpl: Sync + HaveRDBPool + HaveUserRepository {}

#[async_trait]
impl<T: FindUserByNameUsecaseImpl> FindUserByNameUsecase for T {
    async fn find_user_by_name(&self, name: &str) -> UsecaseResult<Option<User>> {
        let mut conn = self.get_rdb_pool().acquire().await?;
        let user = self.user_repo().find_by_name(&mut conn, name).await?;
        Ok(user)
    }
}
