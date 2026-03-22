use {{ crate_name }}_http_core::routes::routes;
use {{ crate_name }}_http_core::state::AppState;
use {{ crate_name }}_infra::rdb::{MySqlRDBPool, build_database_connection_options};
use {{ crate_name }}_infra::services::manager::ServiceManagerInfra;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "info,tower_http=debug,axum::rejection=trace");
        }
    }
    tracing_subscriber::fmt::init();

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .connect_with(build_database_connection_options())
        .await
        .expect("failed to connect db");

    let secret = if let Ok(secret) = std::env::var("SESSION_SECRETKEY") {
        secret.into_bytes()
    } else {
        panic!("please set SESSION_SECRETKEY");
    };

    let service = ServiceManagerInfra::new(MySqlRDBPool::new(pool));

    let app = routes()
        .with_state(AppState {
            service,
            session_key: axum_extra::extract::cookie::Key::derive_from(&secret),
        })
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // HTTPサーバ起動
    if let Some(tcp_listener) = listenfd::ListenFd::from_env().take_tcp_listener(0)? {
        let listener = TcpListener::from_std(tcp_listener).unwrap();
        axum::serve(listener, app.into_make_service()).await?;
    } else {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, app.into_make_service()).await?;
    }

    Ok(())
}
