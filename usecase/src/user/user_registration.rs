#[cfg(test)]
mod tests;

use crate::UsecaseResult;
use async_trait::async_trait;
use {{ crate_name }}_domain::models::user::{CreateUser, User, UserName};
use {{ crate_name }}_domain::rdb::HaveRDBPool;
use {{ crate_name }}_domain::repos::user_repository::{HaveUserRepository, UserRepository};

#[async_trait]
pub trait UserRegistrationUsecase {
    async fn register_user(&self, user: &CreateUser) -> UsecaseResult<User>;
}

pub trait UserRegistrationUsecaseImpl: Sync + HaveRDBPool + HaveUserRepository {}

#[async_trait]
impl<T: UserRegistrationUsecaseImpl> UserRegistrationUsecase for T {
    async fn register_user(&self, user: &CreateUser) -> UsecaseResult<User> {
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
}
