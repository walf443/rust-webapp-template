use crate::usecase::user::UserUsecasesInfra;
use {{ crate_name }}_usecase::manager::UsecaseManager;
use {{ crate_name }}_usecase::user::HaveUserUsecases;

#[derive(Clone)]
pub struct UsecaseManagerInfra {
    user: UserUsecasesInfra,
}

impl UsecaseManagerInfra {
    pub fn new(db_pool: crate::rdb::MySqlRDBPool) -> Self {
        Self {
            user: UserUsecasesInfra::new(db_pool.clone()),
        }
    }
}

impl HaveUserUsecases for UsecaseManagerInfra {
    type User = UserUsecasesInfra;

    fn user(&self) -> &Self::User {
        &self.user
    }
}

impl UsecaseManager for UsecaseManagerInfra {}
