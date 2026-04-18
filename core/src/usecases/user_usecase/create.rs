use crate::models::user::{CreateUser, UserId};
use crate::repos::ReposError::TestError;
use crate::repos::manager::tests::MockRepositoryManager;
use crate::usecases::user_usecase::UserUsecase;
use fake::{Fake, Faker};

#[tokio::test]
async fn user_repo_create_fail() {
    let mut usecase = MockRepositoryManager::new();
    let user: CreateUser = Faker.fake();

    let got_user = user.clone();
    usecase
        .mock_user_repo
        .expect_create()
        .withf(move |_, u| u == &got_user)
        .returning(move |_, _| Err(TestError));

    let result = usecase.create(&user).await;
    assert!(result.is_err())
}

#[tokio::test]
async fn success_case() {
    let mut usecase = MockRepositoryManager::new();
    let user: CreateUser = Faker.fake();

    let expect_user_id: UserId = Faker.fake();
    let got_user = user.clone();
    let uid = expect_user_id.clone();
    usecase
        .mock_user_repo
        .expect_create()
        .withf(move |_, u| u == &got_user)
        .returning(move |_, _| Ok(uid.clone()));

    usecase
        .mock_user_repo
        .expect_hash_password()
        .returning(|_| Ok(Faker.fake()));

    let u = usecase.create(&user).await.unwrap();
    assert_eq!(u.id, expect_user_id)
}
