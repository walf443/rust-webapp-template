use crate::test_helper::MockRepositoryManager;
use crate::user::user_registration::{RegisterUserInput, UserRegistrationUsecase};
use fake::{Fake, Faker};
use {{ crate_name }}_domain::models::user::UserId;
use {{ crate_name }}_domain::repos::ReposError::TestError;

#[tokio::test]
async fn user_repo_create_fail() {
    let mut usecase = MockRepositoryManager::new();
    let input: RegisterUserInput = Faker.fake();

    let expected = input.clone();
    usecase
        .mock_user_repo
        .expect_create()
        .withf(move |_, u| {
            u.name == expected.name
                && u.display_name == expected.display_name
                && u.description == expected.description
        })
        .returning(move |_, _| Err(TestError));

    let result = usecase.register_user(&input).await;
    assert!(result.is_err())
}

#[tokio::test]
async fn success_case() {
    let mut usecase = MockRepositoryManager::new();
    let input: RegisterUserInput = Faker.fake();

    let expect_user_id: UserId = Faker.fake();
    let uid = expect_user_id.clone();
    let expected = input.clone();
    usecase
        .mock_user_repo
        .expect_create()
        .withf(move |_, u| {
            u.name == expected.name
                && u.display_name == expected.display_name
                && u.description == expected.description
        })
        .returning(move |_, _| Ok(uid.clone()));

    let u = usecase.register_user(&input).await.unwrap();
    assert_eq!(u.id, expect_user_id)
}
