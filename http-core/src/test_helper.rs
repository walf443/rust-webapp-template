use crate::state::AppState;
use async_trait::async_trait;
use {{ crate_name }}_core::models::user::{CreateUser, User, UserId};
use {{ crate_name }}_core::usecases::UsecaseResult;
use {{ crate_name }}_core::usecases::manager::UsecaseManager;
use {{ crate_name }}_core::usecases::user_usecase::{HaveUserUsecase, UserUsecase};

#[derive(Clone)]
pub struct TestUserUsecase;

#[async_trait]
impl UserUsecase for TestUserUsecase {
    async fn create(&self, _user: &CreateUser) -> UsecaseResult<User> {
        unimplemented!()
    }
    async fn find(&self, _id: &UserId) -> UsecaseResult<Option<User>> {
        unimplemented!()
    }
    async fn find_by_name(&self, _name: &str) -> UsecaseResult<Option<User>> {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct TestUsecaseManager;

impl HaveUserUsecase for TestUsecaseManager {
    type Usecase = TestUserUsecase;
    fn user_usecase(&self) -> &Self::Usecase {
        &TestUserUsecase
    }
}

impl UsecaseManager for TestUsecaseManager {}

pub fn make_test_state<S: UsecaseManager>(
    usecase: S,
) -> (AppState<S>, axum_extra::extract::cookie::Key) {
    let secret = b"test-secret-key-that-is-long-enough-for-derive";
    let key = axum_extra::extract::cookie::Key::derive_from(secret);
    let state = AppState {
        usecase,
        session_key: key.clone(),
    };
    (state, key)
}
