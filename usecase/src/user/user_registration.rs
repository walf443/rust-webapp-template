#[cfg(test)]
mod tests;

use crate::UsecaseResult;
use async_trait::async_trait;
use fake::Dummy;
use {{ crate_name }}_domain::models::HashedPassword;
use {{ crate_name }}_domain::models::user::{CreateUser, User, UserName};
use {{ crate_name }}_domain::rdb::HaveRDBPool;
use {{ crate_name }}_domain::repos::user_repository::{HaveUserRepository, UserRepository};

#[derive(Debug, Dummy, Clone)]
pub struct RegisterUserInput {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub password: String,
}

#[async_trait]
pub trait UserRegistrationUsecase {
    async fn register_user(&self, input: &RegisterUserInput) -> UsecaseResult<User>;
}

pub trait UserRegistrationUsecaseImpl: Sync + HaveRDBPool + HaveUserRepository {}

#[async_trait]
impl<T: UserRegistrationUsecaseImpl> UserRegistrationUsecase for T {
    async fn register_user(&self, input: &RegisterUserInput) -> UsecaseResult<User> {
        let hashed_password = HashedPassword::from_plain(&input.password)?;

        let create_user = CreateUser {
            name: input.name.clone(),
            display_name: input.display_name.clone(),
            description: input.description.clone(),
            password: hashed_password,
        };

        let mut tx = self.get_rdb_pool().begin().await?;

        let user_id = self.user_repo().create(&mut tx, &create_user).await?;

        tx.commit().await?;

        Ok(User {
            id: user_id,
            name: UserName::new(create_user.name),
            display_name: Some(create_user.display_name),
            description: Some(create_user.description),
        })
    }
}
