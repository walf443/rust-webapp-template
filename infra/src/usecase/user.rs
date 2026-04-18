use crate::repos::user_repository::UserRepositoryInfra;
use {{ crate_name }}_domain::rdb::{HaveRDBPool, RDBPool};
use {{ crate_name }}_domain::repos::user_repository::HaveUserRepository;
use {{ crate_name }}_usecase::user::find_user::FindUserUsecaseImpl;
use {{ crate_name }}_usecase::user::find_user_by_name::FindUserByNameUsecaseImpl;
use {{ crate_name }}_usecase::user::user_registration::UserRegistrationUsecaseImpl;

#[derive(Clone)]
pub struct UserUsecasesInfra {
    db_pool: crate::rdb::MySqlRDBPool,
    user_repo: UserRepositoryInfra,
}

impl UserUsecasesInfra {
    pub fn new(db_pool: crate::rdb::MySqlRDBPool) -> Self {
        Self {
            db_pool,
            user_repo: UserRepositoryInfra {},
        }
    }
}

impl HaveRDBPool for UserUsecasesInfra {
    fn get_rdb_pool(&self) -> &dyn RDBPool {
        &self.db_pool
    }
}

impl HaveUserRepository for UserUsecasesInfra {
    type Repo = UserRepositoryInfra;

    fn user_repo(&self) -> &Self::Repo {
        &self.user_repo
    }
}

impl FindUserUsecaseImpl for UserUsecasesInfra {}
impl FindUserByNameUsecaseImpl for UserUsecasesInfra {}
impl UserRegistrationUsecaseImpl for UserUsecasesInfra {}
