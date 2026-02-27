use std::{net::SocketAddr, sync::Arc};

use axum::{Router, middleware::from_fn, routing::get};
use common::{
    http::{fallback::handle_404, response::ApiResponse},
    middleware::{CorsPolicy, TrackingConfig, make_cors_middleware, tracking_middleware},
};
use config::{loader::ConfigLoader, sources::dotenv::DotenvSource};
use infrastructure::{DatabaseConfig, DbPool, RedisConfig, RedisPool};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    db: Arc<DbPool>,
    redis: Arc<RedisPool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let loader = ConfigLoader::new().with_service_env(DotenvSource::try_from_file(".env")?);

    let database_config = DatabaseConfig::from_loader(&loader)?;
    let redis_config = RedisConfig::from_loader(&loader)?;

    let db = Arc::new(DbPool::new(&database_config).await?);
    let redis = Arc::new(RedisPool::from_config(&redis_config).await?);

    info!(db_url = %database_config.url, "postgres pool initialized");
    info!(redis_url = %redis_config.url, "redis client initialized");

    let shared_infra = AppState { db, redis };
    let db_ptr = Arc::as_ptr(&shared_infra.db) as usize;
    let redis_ptr = Arc::as_ptr(&shared_infra.redis) as usize;
    info!(
        shared_db = format_args!("0x{db_ptr:x}"),
        shared_redis = format_args!("0x{redis_ptr:x}"),
        "shared infrastructure state initialized"
    );
    let app = build_router();

    let address = std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let addr: SocketAddr = address.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!(%addr, "gateway started");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    drop(shared_infra);

    Ok(())
}

fn build_router() -> Router {
    let cors = make_cors_middleware(CorsPolicy::new().allow_all_origins());

    Router::new()
        .route("/", get(live))
        .route("/health", get(health))
        .nest("/api/v1/identity", identity::router())
        .nest("/api/v1/order", order::router())
        .nest("/api/v1/escrow", escrow::router())
        .nest("/api/v1/catalog", catalog::router())
        .nest("/api/v1/shipment", shipment::router())
        .nest("/api/v1/dispute", dispute::router())
        .nest("/api/v1/trust", trust::router())
        .nest("/api/v1/risk", risk::router())
        .nest("/api/v1/evidence", evidence::router())
        .nest("/api/v1/notification", notification::router())
        .nest("/api/v1/messaging", messaging::router())
        .nest("/api/v1/analytics", analytics::router())
        .fallback(handle_404)
        .layer(from_fn(cors))
        .layer(from_fn(|req, next| {
            Box::pin(tracking_middleware(req, next, TrackingConfig::default()))
        }))
}

async fn live() -> ApiResponse {
    ApiResponse::success_message("TrustFlow is Live")
}

async fn health() -> ApiResponse {
    ApiResponse::success_message("gateway healthy")
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
