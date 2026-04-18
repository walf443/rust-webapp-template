use crate::repos::user_repository::UserRepositoryInfra;
use {{ crate_name }}_domain::rdb::{HaveRDBPool, RDBPool};
use {{ crate_name }}_domain::repos::user_repository::HaveUserRepository;
use {{ crate_name }}_usecase::user_usecase::UserUsecaseImpl;

#[derive(Clone)]
pub struct UserUsecaseInfra {
    db_pool: crate::rdb::MySqlRDBPool,
    user_repo: UserRepositoryInfra,
}

impl UserUsecaseInfra {
    pub fn new(db_pool: crate::rdb::MySqlRDBPool) -> Self {
        Self {
            db_pool,
            user_repo: UserRepositoryInfra {},
        }
    }
}

impl HaveRDBPool for UserUsecaseInfra {
    fn get_rdb_pool(&self) -> &dyn RDBPool {
        &self.db_pool
    }
}

impl HaveUserRepository for UserUsecaseInfra {
    type Repo = UserRepositoryInfra;

    fn user_repo(&self) -> &Self::Repo {
        &self.user_repo
    }
}

impl UserUsecaseImpl for UserUsecaseInfra {}
