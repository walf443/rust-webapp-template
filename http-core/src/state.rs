use {{ crate_name }}_core::services::manager::ServiceManager;

#[derive(Clone)]
pub struct AppState<S: ServiceManager> {
    pub service: S,
    pub session_key: axum_extra::extract::cookie::Key,
}
impl<S: ServiceManager> axum::extract::FromRef<AppState<S>> for axum_extra::extract::cookie::Key {
    fn from_ref(state: &AppState<S>) -> Self {
        state.session_key.clone()
    }
}
