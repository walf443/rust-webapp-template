use {{ crate_name }}_core::models::user::{User, UserId, UserName};

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct UserRow {
    pub id: UserId,
    pub name: UserName,
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[sqlx(default, rename = "password")]
    pub hashed_password: Option<String>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            name: row.name,
            display_name: row.display_name,
            description: row.description,
            hashed_password: row.hashed_password,
        }
    }
}
