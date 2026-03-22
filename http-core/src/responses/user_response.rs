use crate::responses::ResponseResult;
use {{ crate_name }}_core::models::user::{User, UserId, UserName};
use {{ crate_name }}_core::services::manager::ServiceManager;

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
    pub async fn build_by_service<S: ServiceManager>(
        _service: &S,
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
