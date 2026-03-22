use crate::routes::user_routes::user_routes;
use crate::state::AppState;
use axum::Router;
use {{ crate_name }}_core::services::manager::ServiceManager;

pub mod user_routes;

pub fn routes<S: ServiceManager + 'static>() -> Router<AppState<S>> {
    axum::Router::new().nest("/api/user/", user_routes())
}
