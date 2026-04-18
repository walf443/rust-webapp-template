use std::any::Any;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, thiserror::Error)]
pub enum RDBError {
    #[error("{0}")]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

/// Internal trait for connection functionality. Implemented in infra.
pub trait RDBConnectionInner: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Commit the transaction. Default is no-op (for non-transaction connections).
    fn commit_boxed(self: Box<Self>) -> Pin<Box<dyn Future<Output = Result<(), RDBError>> + Send>> {
        Box::pin(async { Ok(()) })
    }
}

/// Concrete database connection handle.
/// Used for both pooled connections and transactions.
pub struct RDBConnection {
    inner: Box<dyn RDBConnectionInner>,
}

impl RDBConnection {
    pub fn new(inner: Box<dyn RDBConnectionInner>) -> Self {
        Self { inner }
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self.inner.as_any_mut()
    }

    /// Commit the transaction. No-op for non-transaction connections.
    pub async fn commit(self) -> Result<(), RDBError> {
        self.inner.commit_boxed().await
    }
}

/// Abstract connection pool trait.
#[async_trait::async_trait]
pub trait RDBPool: Send + Sync {
    async fn acquire(&self) -> Result<RDBConnection, RDBError>;
    async fn begin(&self) -> Result<RDBConnection, RDBError>;
}

pub trait HaveRDBPool {
    fn get_rdb_pool(&self) -> &dyn RDBPool;
}

#[cfg(any(feature = "test", test))]
pub mod test_utils {
    use super::*;

    pub struct MockRDBConnectionInner;

    impl RDBConnectionInner for MockRDBConnectionInner {
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    pub struct MockRDBPool;

    #[async_trait::async_trait]
    impl RDBPool for MockRDBPool {
        async fn acquire(&self) -> Result<RDBConnection, RDBError> {
            Ok(RDBConnection::new(Box::new(MockRDBConnectionInner)))
        }

        async fn begin(&self) -> Result<RDBConnection, RDBError> {
            Ok(RDBConnection::new(Box::new(MockRDBConnectionInner)))
        }
    }
}
