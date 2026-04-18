use crate::repos::user_repository::UserRepositoryInfra;
use fake::{Fake, Faker};
use {{ crate_name }}_domain::models::HashedPassword;
use {{ crate_name }}_domain::models::user::{User, UserId};
use {{ crate_name }}_domain::rdb::RDBPool;
use {{ crate_name }}_domain::repos::user_repository::UserRepository;

#[tokio::test]
async fn found_case() {
    let pool = crate::rdb::get_test_pool().await;
    let mut conn = pool.begin().await.unwrap();

    let user_id: UserId = Faker.fake();
    let repo = UserRepositoryInfra {};

    let mut user: User = Faker.fake();
    user.id = user_id.clone();
    user.display_name = Some(Faker.fake());
    user.description = Some(Faker.fake());
    let hashed_password: HashedPassword = Faker.fake();

    let mysql_conn = crate::rdb::get_mysql_conn(&mut conn);
    sqlx::query(
        "INSERT INTO users (id, name, display_name, description, password) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&user.display_name)
    .bind(&user.description)
    .bind(hashed_password.as_str())
    .execute(mysql_conn)
    .await
    .unwrap();

    let got = repo.find(&mut conn, &user_id).await.unwrap();
    assert!(got.is_some());
    let got = got.unwrap();
    assert_eq!(got.id, user_id);
    assert_eq!(got.name, user.name);
    assert_eq!(got.display_name, user.display_name);
    assert_eq!(got.description, user.description);
}

#[tokio::test]
async fn not_found_case() {
    let pool = crate::rdb::get_test_pool().await;
    let mut conn = pool.begin().await.unwrap();

    let user_id: UserId = Faker.fake();
    let repo = UserRepositoryInfra {};

    let user = repo.find(&mut conn, &user_id).await.unwrap();
    assert!(user.is_none());
}
