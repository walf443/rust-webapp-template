use crate::rdb::HaveRDBPool;
use crate::repos::user_repository::HaveUserRepository;

pub trait RepositoryManager: Sync + HaveRDBPool + HaveUserRepository {}
