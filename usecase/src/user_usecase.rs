#[cfg(test)]
mod create;

use crate::UsecaseResult;
use async_trait::async_trait;
use {{ crate_name }}_domain::models::user::{CreateUser, User, UserId, UserName};
use {{ crate_name }}_domain::rdb::HaveRDBPool;
use {{ crate_name }}_domain::repos::user_repository::{HaveUserRepository, UserRepository};

#[async_trait]
pub trait UserUsecase {
    async fn create(&self, user: &CreateUser) -> UsecaseResult<User>;
    async fn find(&self, id: &UserId) -> UsecaseResult<Option<User>>;
    async fn find_by_name(&self, name: &str) -> UsecaseResult<Option<User>>;
}

pub trait HaveUserUsecase {
    type Usecase: UserUsecase;

    fn user_usecase(&self) -> &Self::Usecase;
}

pub trait UserUsecaseImpl: Sync + HaveRDBPool + HaveUserRepository {}

#[async_trait]
impl<T: UserUsecaseImpl> UserUsecase for T {
    async fn create(&self, user: &CreateUser) -> UsecaseResult<User> {
        let mut tx = self.get_rdb_pool().begin().await?;

        let user_id = self.user_repo().create(&mut tx, user).await?;

        let hashed_password = self.user_repo().hash_password(&user.password)?;

        Ok(User {
            id: user_id,
            name: UserName::new(user.name.clone()),
            display_name: Some(user.display_name.clone()),
            description: Some(user.description.clone()),
            hashed_password: Some(hashed_password),
        })
    }

    async fn find(&self, id: &UserId) -> UsecaseResult<Option<User>> {
        let mut conn = self.get_rdb_pool().acquire().await?;
        let user = self.user_repo().find(&mut conn, id).await?;
        Ok(user)
    }

    async fn find_by_name(&self, name: &str) -> UsecaseResult<Option<User>> {
        let mut conn = self.get_rdb_pool().acquire().await?;
        let user = self.user_repo().find_by_name(&mut conn, name).await?;
        Ok(user)
    }
}
