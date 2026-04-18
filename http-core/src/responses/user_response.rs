use crate::responses::ResponseResult;
use {{ crate_name }}_core::models::user::{User, UserId, UserName};
use {{ crate_name }}_core::usecases::manager::UsecaseManager;

#[derive(Debug, serde::Serialize)]
pub struct UserResponse {
    pub id: UserId,
    pub name: UserName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UserResponse {
    pub async fn build_by_usecase<S: UsecaseManager>(
        _usecase: &S,
        user: &User,
    ) -> ResponseResult<Self> {
        Ok(Self {
            id: user.id.clone(),
            name: user.name.clone(),
            display_name: user.display_name.clone(),
            description: user.description.clone(),
        })
    }
}
