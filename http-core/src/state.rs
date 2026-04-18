use {{ crate_name }}_core::usecases::manager::UsecaseManager;

#[derive(Clone)]
pub struct AppState<S: UsecaseManager> {
    pub usecase: S,
    pub session_key: axum_extra::extract::cookie::Key,
}
impl<S: UsecaseManager> axum::extract::FromRef<AppState<S>> for axum_extra::extract::cookie::Key {
    fn from_ref(state: &AppState<S>) -> Self {
        state.session_key.clone()
    }
}
