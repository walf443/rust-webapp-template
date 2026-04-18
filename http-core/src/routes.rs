use crate::routes::user_routes::user_routes;
use crate::state::AppState;
use axum::Router;
use {{ crate_name }}_usecase::manager::UsecaseManager;

pub mod user_routes;

pub fn routes<S: UsecaseManager + 'static>() -> Router<AppState<S>> {
    axum::Router::new().nest("/api/user/", user_routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{TestUsecaseManager, make_test_state};

    #[test]
    fn router_builds_without_panic() {
        let (state, _) = make_test_state(TestUsecaseManager);
        let _app: axum::Router = routes().with_state(state);
    }
}
