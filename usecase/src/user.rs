pub mod find_user;
pub mod find_user_by_name;
pub mod user_registration;

pub use find_user::FindUserUsecase;
pub use find_user_by_name::FindUserByNameUsecase;
pub use user_registration::UserRegistrationUsecase;

pub trait HaveUserUsecases {
    type User: FindUserUsecase + FindUserByNameUsecase + UserRegistrationUsecase;

    fn user(&self) -> &Self::User;
}
