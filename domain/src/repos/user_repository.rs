use crate::models::user::{CreateUser, User, UserId};
use crate::rdb::RDBConnection;
use crate::repos::Result;
use async_trait::async_trait;

#[cfg_attr(any(feature = "test", test), mockall::automock)]
#[async_trait]
pub trait UserRepository {
    async fn create(&self, conn: &mut RDBConnection, user: &CreateUser) -> Result<UserId>;

    async fn find(&self, conn: &mut RDBConnection, id: &UserId) -> Result<Option<User>>;
    async fn find_all(&self, conn: &mut RDBConnection) -> Result<Vec<User>>;
    async fn find_id_by_name(&self, conn: &mut RDBConnection, name: &str)
    -> Result<Option<UserId>>;
    async fn find_by_name(&self, conn: &mut RDBConnection, name: &str) -> Result<Option<User>>;
}

pub trait HaveUserRepository {
    type Repo: UserRepository;

    fn user_repo(&self) -> &Self::Repo;
}
