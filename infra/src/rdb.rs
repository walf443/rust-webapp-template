use std::any::Any;
use std::future::Future;
use std::pin::Pin;

use sqlx::mysql::MySql;
use sqlx::pool::PoolConnection;
use sqlx::{MySqlConnection, MySqlPool};
use {{ crate_name }}_core::rdb::{RDBConnection, RDBConnectionInner, RDBError, RDBPool};

/// Wrapper around sqlx PoolConnection for the RDBConnectionInner trait.
struct MySqlPooledConnection(PoolConnection<MySql>);

impl RDBConnectionInner for MySqlPooledConnection {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut self.0
    }
}

/// Wrapper around sqlx Transaction for the RDBConnectionInner trait (with commit).
struct MySqlTransactionConnection(sqlx::Transaction<'static, MySql>);

impl RDBConnectionInner for MySqlTransactionConnection {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut self.0
    }

    fn commit_boxed(self: Box<Self>) -> Pin<Box<dyn Future<Output = Result<(), RDBError>> + Send>> {
        Box::pin(async {
            self.0
                .commit()
                .await
                .map_err(|e| RDBError::Other(Box::new(e)))
        })
    }
}

/// Wrapper around MySqlPool implementing the abstract RDBPool trait.
#[derive(Clone)]
pub struct MySqlRDBPool(MySqlPool);

impl MySqlRDBPool {
    pub fn new(pool: MySqlPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl RDBPool for MySqlRDBPool {
    async fn acquire(&self) -> Result<RDBConnection, RDBError> {
        let conn = self
            .0
            .acquire()
            .await
            .map_err(|e| RDBError::Other(Box::new(e)))?;
        Ok(RDBConnection::new(Box::new(MySqlPooledConnection(conn))))
    }

    async fn begin(&self) -> Result<RDBConnection, RDBError> {
        let tx = self
            .0
            .begin()
            .await
            .map_err(|e| RDBError::Other(Box::new(e)))?;
        Ok(RDBConnection::new(Box::new(MySqlTransactionConnection(tx))))
    }
}

/// Helper to downcast an abstract RDBConnection to a concrete &mut MySqlConnection.
/// Works for both pooled connections and transactions.
pub(crate) fn get_mysql_conn(conn: &mut RDBConnection) -> &mut MySqlConnection {
    let any = conn.as_any_mut();
    if any.is::<PoolConnection<MySql>>() {
        any.downcast_mut::<PoolConnection<MySql>>()
            .unwrap()
            .as_mut()
    } else if any.is::<sqlx::Transaction<'static, MySql>>() {
        any.downcast_mut::<sqlx::Transaction<'static, MySql>>()
            .unwrap()
            .as_mut()
    } else {
        panic!("Expected MySQL connection type (PoolConnection or Transaction)")
    }
}

/// Convert sqlx::Error to RDBError for use with ? operator in infra repos.
pub(crate) fn sqlx_err(e: sqlx::Error) -> RDBError {
    RDBError::Other(Box::new(e))
}

#[cfg(test)]
pub(crate) async fn get_test_pool() -> MySqlRDBPool {
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::mysql::Mysql;
    use tokio::sync::OnceCell;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Mysql>,
        pool: MySqlRDBPool,
    }

    // Safety: Send is required for OnceCell but ContainerAsync may not impl Send
    // in all versions. The container is only used to keep it alive.
    unsafe impl Send for TestContext {}
    unsafe impl Sync for TestContext {}

    static TEST_CONTEXT: OnceCell<TestContext> = OnceCell::const_new();

    let ctx = TEST_CONTEXT
        .get_or_init(|| async {
            let container = Mysql::default()
                .with_init_sql(include_str!("../schema.sql").to_string().into_bytes())
                .start()
                .await
                .expect("Failed to start MySQL container");

            let host = container.get_host().await.unwrap();
            let port = container.get_host_port_ipv4(3306).await.unwrap();

            let options = sqlx::mysql::MySqlConnectOptions::new()
                .host(&host.to_string())
                .port(port)
                .username("root")
                .database("test");

            let pool = sqlx::mysql::MySqlPoolOptions::new()
                .connect_with(options)
                .await
                .expect("Failed to connect to test MySQL");

            TestContext {
                _container: container,
                pool: MySqlRDBPool::new(pool),
            }
        })
        .await;

    ctx.pool.clone()
}

/// Database connection configuration from environment variables.
pub fn build_database_connection_options() -> sqlx::mysql::MySqlConnectOptions {
    let mut options = sqlx::mysql::MySqlConnectOptions::new()
        .host("127.0.0.1")
        .port(3306);

    if let Ok(host) = std::env::var("RDB_HOST") {
        options = options.host(&host);
    }
    if let Some(port) = std::env::var("RDB_PORT")
        .ok()
        .and_then(|port_str| port_str.parse().ok())
    {
        options = options.port(port);
    }
    if let Ok(user) = std::env::var("RDB_USER") {
        options = options.username(&user);
    }
    if let Ok(password) = std::env::var("RDB_PASSWORD") {
        options = options.password(&password);
    }
    if let Ok(database) = std::env::var("RDB_NAME") {
        options = options.database(&database);
    }
    options
}
