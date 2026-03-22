use crate::repos::user_repository::UserRepositoryInfra;
use {{ crate_name }}_core::rdb::{HaveRDBPool, RDBPool};
use {{ crate_name }}_core::repos::user_repository::HaveUserRepository;
use {{ crate_name }}_core::services::user_service::UserServiceImpl;

#[derive(Clone)]
pub struct UserServiceInfra {
    db_pool: crate::rdb::MySqlRDBPool,
    user_repo: UserRepositoryInfra,
}

impl UserServiceInfra {
    pub fn new(db_pool: crate::rdb::MySqlRDBPool) -> Self {
        Self {
            db_pool,
            user_repo: UserRepositoryInfra {},
        }
    }
}

impl HaveRDBPool for UserServiceInfra {
    fn get_rdb_pool(&self) -> &dyn RDBPool {
        &self.db_pool
    }
}

impl HaveUserRepository for UserServiceInfra {
    type Repo = UserRepositoryInfra;

    fn user_repo(&self) -> &Self::Repo {
        &self.user_repo
    }
}

impl UserServiceImpl for UserServiceInfra {}
