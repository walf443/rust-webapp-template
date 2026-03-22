use crate::services::user_service::UserServiceInfra;
use {{ crate_name }}_core::services::manager::ServiceManager;
use {{ crate_name }}_core::services::user_service::HaveUserService;

#[derive(Clone)]
pub struct ServiceManagerInfra {
    user_service: UserServiceInfra,
}

impl ServiceManagerInfra {
    pub fn new(db_pool: crate::rdb::MySqlRDBPool) -> Self {
        Self {
            user_service: UserServiceInfra::new(db_pool.clone()),
        }
    }
}

impl HaveUserService for ServiceManagerInfra {
    type Service = UserServiceInfra;

    fn user_service(&self) -> &Self::Service {
        &self.user_service
    }
}

impl ServiceManager for ServiceManagerInfra {}
