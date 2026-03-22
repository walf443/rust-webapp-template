use crate::services::user_service::HaveUserService;

pub trait ServiceManager: Send + Sync + Clone + HaveUserService {}
