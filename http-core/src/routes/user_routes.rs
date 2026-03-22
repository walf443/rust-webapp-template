use crate::error::Error;
use crate::responses::user_response::UserResponse;
use crate::state::AppState;
use axum::Router;
use axum::extract::{Path, State};
use {{ crate_name }}_core::services::manager::ServiceManager;
use {{ crate_name }}_core::services::user_service::UserService;

pub fn user_routes<S: ServiceManager + 'static>() -> Router<AppState<S>> {
    Router::new().route("/:username", axum::routing::get(get_user_handler::<S>))
}

// GET /api/user/:username
pub async fn get_user_handler<S: ServiceManager>(
    State(AppState { service, .. }): State<AppState<S>>,
    Path((username,)): Path<(String,)>,
) -> Result<axum::Json<UserResponse>, Error> {
    let user_model = service
        .user_service()
        .find_by_name(&username)
        .await?
        .ok_or(Error::NotFound(
            "not found user that has the given username".into(),
        ))?;

    let user = UserResponse::build_by_service(&service, &user_model).await?;

    Ok(axum::Json(user))
}
