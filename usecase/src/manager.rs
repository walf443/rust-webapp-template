use crate::user::HaveUserUsecases;

pub trait UsecaseManager: Send + Sync + Clone + HaveUserUsecases {}
