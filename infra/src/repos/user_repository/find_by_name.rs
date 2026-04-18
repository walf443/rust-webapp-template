use crate::repos::user_repository::UserRepositoryInfra;
use fake::{Fake, Faker};
use {{ crate_name }}_domain::models::user::{CreateUser, UserId};
use {{ crate_name }}_domain::rdb::RDBPool;
use {{ crate_name }}_domain::repos::user_repository::UserRepository;

#[tokio::test]
async fn not_found_case() {
    let pool = crate::rdb::get_test_pool().await;
    let mut conn = pool.begin().await.unwrap();

    let repo = UserRepositoryInfra {};
    let name: String = Faker.fake();
    let got_user = repo.find_by_name(&mut conn, &name).await.unwrap();
    assert!(got_user.is_none())
}

#[tokio::test]
async fn found_case() {
    let pool = crate::rdb::get_test_pool().await;
    let mut conn = pool.begin().await.unwrap();

    let user: CreateUser = Faker.fake();

    let mysql_conn = crate::rdb::get_mysql_conn(&mut conn);
    let result = sqlx::query(
        "INSERT INTO users (name, description, display_name, password) VALUES (?, ?, ?, ?)",
    )
    .bind(&user.name)
    .bind(&user.description)
    .bind(&user.display_name)
    .bind(&user.password)
    .execute(mysql_conn)
    .await
    .unwrap();
    let user_id = result.last_insert_id() as i64;
    let user_id = UserId::new(user_id);

    let repo = UserRepositoryInfra {};
    let got_user = repo.find_by_name(&mut conn, &user.name).await.unwrap();
    assert!(got_user.is_some());
    assert_eq!(got_user.unwrap().id, user_id)
}
