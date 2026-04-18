use crate::user_usecase::HaveUserUsecase;

pub trait UsecaseManager: Send + Sync + Clone + HaveUserUsecase {}
