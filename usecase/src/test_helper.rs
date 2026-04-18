use crate::user::find_user::FindUserUsecaseImpl;
use crate::user::find_user_by_name::FindUserByNameUsecaseImpl;
use crate::user::user_registration::UserRegistrationUsecaseImpl;
use {{ crate_name }}_domain::rdb::test_utils::MockRDBPool;
use {{ crate_name }}_domain::rdb::{HaveRDBPool, RDBPool};
use {{ crate_name }}_domain::repos::user_repository::{HaveUserRepository, MockUserRepository};

pub(crate) struct MockRepositoryManager {
    db_pool: MockRDBPool,
    pub(crate) mock_user_repo: MockUserRepository,
}

impl Default for MockRepositoryManager {
    fn default() -> Self {
        Self {
            db_pool: MockRDBPool,
            mock_user_repo: Default::default(),
        }
    }
}

impl MockRepositoryManager {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl HaveRDBPool for MockRepositoryManager {
    fn get_rdb_pool(&self) -> &dyn RDBPool {
        &self.db_pool
    }
}

impl HaveUserRepository for MockRepositoryManager {
    type Repo = MockUserRepository;

    fn user_repo(&self) -> &Self::Repo {
        &self.mock_user_repo
    }
}

impl FindUserUsecaseImpl for MockRepositoryManager {}
impl FindUserByNameUsecaseImpl for MockRepositoryManager {}
impl UserRegistrationUsecaseImpl for MockRepositoryManager {}
