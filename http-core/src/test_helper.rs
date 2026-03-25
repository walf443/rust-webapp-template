use crate::state::AppState;
use {{ crate_name }}_core::models::user::{CreateUser, User, UserId};
use {{ crate_name }}_core::services::manager::ServiceManager;
use {{ crate_name }}_core::services::user_service::{HaveUserService, UserService};
use {{ crate_name }}_core::services::ServiceResult;
use async_trait::async_trait;

#[derive(Clone, Default)]
pub struct TestUserService;

#[async_trait]
impl UserService for TestUserService {
    async fn create(&self, _user: &CreateUser) -> ServiceResult<User> {
        unimplemented!()
    }
    async fn find(&self, _id: &UserId) -> ServiceResult<Option<User>> {
        unimplemented!()
    }
    async fn find_by_name(&self, _name: &str) -> ServiceResult<Option<User>> {
        Ok(None)
    }
}

#[derive(Clone, Default)]
pub struct TestServiceManager;

impl HaveUserService for TestServiceManager {
    type Service = TestUserService;
    fn user_service(&self) -> &Self::Service {
        &TestUserService
    }
}

impl ServiceManager for TestServiceManager {}

pub fn make_test_state<S: ServiceManager>(service: S) -> (AppState<S>, axum_extra::extract::cookie::Key) {
    let secret = b"test-secret-key-that-is-long-enough-for-derive";
    let key = axum_extra::extract::cookie::Key::derive_from(secret);
    let state = AppState {
        service,
        session_key: key.clone(),
    };
    (state, key)
}
