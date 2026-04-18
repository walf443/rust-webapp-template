use crate::error::Error;
use crate::responses::user_response::UserResponse;
use crate::state::AppState;
use axum::Router;
use axum::extract::{Path, State};
use {{ crate_name }}_usecase::manager::UsecaseManager;
use {{ crate_name }}_usecase::user_usecase::UserUsecase;

pub fn user_routes<S: UsecaseManager + 'static>() -> Router<AppState<S>> {
    Router::new().route("/{username}", axum::routing::get(get_user_handler::<S>))
}

// GET /api/user/{username}
pub async fn get_user_handler<S: UsecaseManager>(
    State(AppState { usecase, .. }): State<AppState<S>>,
    Path((username,)): Path<(String,)>,
) -> Result<axum::Json<UserResponse>, Error> {
    let user_model = usecase
        .user_usecase()
        .find_by_name(&username)
        .await?
        .ok_or(Error::NotFound(
            "not found user that has the given username".into(),
        ))?;

    let user = UserResponse::build_by_usecase(&usecase, &user_model).await?;

    Ok(axum::Json(user))
}
