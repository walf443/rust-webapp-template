use {{ crate_name }}_domain::models::user::{User, UserId, UserName};

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct UserRow {
    pub id: UserId,
    pub name: UserName,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            name: row.name,
            display_name: row.display_name,
            description: row.description,
        }
    }
}
