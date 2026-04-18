use crate::repos::user_repository::UserRepositoryInfra;
use fake::{Fake, Faker};
use {{ crate_name }}_domain::models::user::{CreateUser, User};
use {{ crate_name }}_domain::rdb::RDBPool;
use {{ crate_name }}_domain::repos::user_repository::UserRepository;

#[tokio::test]
async fn success_case() {
    let pool = crate::rdb::get_test_pool().await;
    let mut conn = pool.begin().await.unwrap();

    let user: CreateUser = Faker.fake();

    let repo = UserRepositoryInfra {};
    let user_id = repo.create(&mut conn, &user).await.unwrap();

    let mysql_conn = crate::rdb::get_mysql_conn(&mut conn);
    let got: User = sqlx::query_as::<_, crate::rows::UserRow>("SELECT * FROM users where id = ?")
        .bind(&user_id)
        .fetch_one(mysql_conn)
        .await
        .unwrap()
        .into();
    assert_eq!(got.id, user_id);
    assert_eq!(got.name.inner(), &user.name);
    assert_eq!(got.description, Some(user.description));
    assert_eq!(got.display_name, Some(user.display_name));
}
