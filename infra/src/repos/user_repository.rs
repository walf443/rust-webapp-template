#[cfg(test)]
mod create;
#[cfg(test)]
mod find;
#[cfg(test)]
mod find_by_name;

use async_trait::async_trait;
use {{ crate_name }}_core::models::user::{CreateUser, User, UserId};
use {{ crate_name }}_core::rdb::RDBConnection;
use {{ crate_name }}_core::repos::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserRepositoryInfra {}

#[async_trait]
impl UserRepository for UserRepositoryInfra {
    async fn create(
        &self,
        conn: &mut RDBConnection,
        user: &CreateUser,
    ) -> {{ crate_name }}_core::repos::Result<UserId> {
        let conn = crate::rdb::get_mysql_conn(conn);
        let hashed_password = self.hash_password(&user.password)?;

        let result = sqlx::query(
            "INSERT INTO users (name, display_name, description, password) VALUES(?, ?, ?, ?)",
        )
        .bind(&user.name)
        .bind(&user.display_name)
        .bind(&user.description)
        .bind(&hashed_password)
        .execute(conn)
        .await
        .map_err(crate::rdb::sqlx_err)?;

        let user_id = result.last_insert_id() as i64;

        Ok(UserId::new(user_id))
    }

    async fn find(
        &self,
        conn: &mut RDBConnection,
        id: &UserId,
    ) -> {{ crate_name }}_core::repos::Result<Option<User>> {
        let conn = crate::rdb::get_mysql_conn(conn);
        let user_model: Option<crate::rows::UserRow> = sqlx::query_as(
            "SELECT id, name, display_name, description, password as hashed_password FROM users WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(conn)
        .await
        .map_err(crate::rdb::sqlx_err)?;

        Ok(user_model.map(User::from))
    }

    async fn find_all(&self, conn: &mut RDBConnection) -> {{ crate_name }}_core::repos::Result<Vec<User>> {
        let conn = crate::rdb::get_mysql_conn(conn);
        let users: Vec<crate::rows::UserRow> = sqlx::query_as(
            "SELECT id, name, display_name, description, password as hashed_password FROM users",
        )
        .fetch_all(conn)
        .await
        .map_err(crate::rdb::sqlx_err)?;

        Ok(users.into_iter().map(User::from).collect())
    }

    async fn find_id_by_name(
        &self,
        conn: &mut RDBConnection,
        name: &str,
    ) -> {{ crate_name }}_core::repos::Result<Option<UserId>> {
        let conn = crate::rdb::get_mysql_conn(conn);
        let user_id: Option<UserId> = sqlx::query_scalar("SELECT id FROM users WHERE name = ?")
            .bind(name)
            .fetch_optional(conn)
            .await
            .map_err(crate::rdb::sqlx_err)?;

        Ok(user_id)
    }

    async fn find_by_name(
        &self,
        conn: &mut RDBConnection,
        name: &str,
    ) -> {{ crate_name }}_core::repos::Result<Option<User>> {
        let conn = crate::rdb::get_mysql_conn(conn);
        let user_model: Option<crate::rows::UserRow> = sqlx::query_as(
            "SELECT id, name, display_name, description, password as hashed_password FROM users WHERE name = ?",
        )
        .bind(name)
        .fetch_optional(conn)
        .await
        .map_err(crate::rdb::sqlx_err)?;

        Ok(user_model.map(User::from))
    }
}
