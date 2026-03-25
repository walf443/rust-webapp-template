use crate::routes::user_routes::user_routes;
use crate::state::AppState;
use axum::Router;
use {{ crate_name }}_core::services::manager::ServiceManager;

pub mod user_routes;

pub fn routes<S: ServiceManager + 'static>() -> Router<AppState<S>> {
    axum::Router::new().nest("/api/user/", user_routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{TestServiceManager, make_test_state};

    #[test]
    fn router_builds_without_panic() {
        let (state, _) = make_test_state(TestServiceManager::default());
        let _app: axum::Router = routes().with_state(state);
    }
}
