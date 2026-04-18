use super::hashed_password::HashedPassword;
use super::id::Id;
use fake::Dummy;

#[derive(Debug, Dummy)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

pub type UserId = Id<User, i64>;

pub type UserName = Id<User, String>;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(any(feature = "test", test), derive(fake::Dummy))]
pub struct CreateUser {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub password: HashedPassword,
}
