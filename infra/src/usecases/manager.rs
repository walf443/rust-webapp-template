use crate::usecases::user_usecase::UserUsecaseInfra;
use {{ crate_name }}_core::usecases::manager::UsecaseManager;
use {{ crate_name }}_core::usecases::user_usecase::HaveUserUsecase;

#[derive(Clone)]
pub struct UsecaseManagerInfra {
    user_usecase: UserUsecaseInfra,
}

impl UsecaseManagerInfra {
    pub fn new(db_pool: crate::rdb::MySqlRDBPool) -> Self {
        Self {
            user_usecase: UserUsecaseInfra::new(db_pool.clone()),
        }
    }
}

impl HaveUserUsecase for UsecaseManagerInfra {
    type Usecase = UserUsecaseInfra;

    fn user_usecase(&self) -> &Self::Usecase {
        &self.user_usecase
    }
}

impl UsecaseManager for UsecaseManagerInfra {}
