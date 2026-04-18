use crate::rdb::HaveRDBPool;
use crate::repos::user_repository::HaveUserRepository;

pub trait RepositoryManager: Sync + HaveRDBPool + HaveUserRepository {}

#[cfg(test)]
pub mod tests {
    use crate::rdb::test_utils::MockRDBPool;
    use crate::rdb::{HaveRDBPool, RDBPool};
    use crate::repos::manager::RepositoryManager;
    use crate::repos::user_repository::{HaveUserRepository, MockUserRepository};
    use crate::usecases::user_usecase::UserUsecaseImpl;

    pub struct MockRepositoryManager {
        db_pool: MockRDBPool,
        pub mock_user_repo: MockUserRepository,
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
        pub fn new() -> Self {
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

    impl RepositoryManager for MockRepositoryManager {}
    impl UserUsecaseImpl for MockRepositoryManager {}
}
