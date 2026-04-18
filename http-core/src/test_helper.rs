use crate::state::AppState;
use async_trait::async_trait;
use {{ crate_name }}_domain::models::user::{User, UserId};
use {{ crate_name }}_usecase::UsecaseResult;
use {{ crate_name }}_usecase::manager::UsecaseManager;
use {{ crate_name }}_usecase::user::user_registration::RegisterUserInput;
use {{ crate_name }}_usecase::user::{
    FindUserByNameUsecase, FindUserUsecase, HaveUserUsecases, UserRegistrationUsecase,
};

#[derive(Clone)]
pub struct TestUserUsecases;

#[async_trait]
impl FindUserUsecase for TestUserUsecases {
    async fn find_user(&self, _id: &UserId) -> UsecaseResult<Option<User>> {
        unimplemented!()
    }
}

#[async_trait]
impl FindUserByNameUsecase for TestUserUsecases {
    async fn find_user_by_name(&self, _name: &str) -> UsecaseResult<Option<User>> {
        Ok(None)
    }
}

#[async_trait]
impl UserRegistrationUsecase for TestUserUsecases {
    async fn register_user(&self, _input: &RegisterUserInput) -> UsecaseResult<User> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct TestUsecaseManager;

impl HaveUserUsecases for TestUsecaseManager {
    type User = TestUserUsecases;
    fn user(&self) -> &Self::User {
        &TestUserUsecases
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
