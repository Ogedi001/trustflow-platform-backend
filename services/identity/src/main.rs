//! Identity Service - Main Entry Point
//!
//! This service handles user identity, authentication, authorization, and verification.

use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

mod api;
mod application;
mod domain;
mod infrastructure;

use api::routes::router;
use application::config::Config;
use application::{ApplicationContext, InfrastructureRef};
use infrastructure::{Infrastructure, InfrastructureConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing::init();

    // Load configuration
    let config = Config::from_env();
    let infra_config = InfrastructureConfig::from_env();

    // Initialize infrastructure
    let infrastructure = Infrastructure::new(infra_config.clone()).await?;

    // Create infrastructure reference for application
    let infra_ref =
        InfrastructureRef::new(infrastructure.db().clone(), infrastructure.redis().clone());

    // Create application context with shared config
    let app_context = ApplicationContext::new(infra_ref, Arc::new(config.clone()));

    // Build router
    let app = router(app_context);

    // Print startup message
    tracing::info!("Identity Service starting on {}", config.server.address());
    tracing::info!("Database URL: {}", infra_config.db.url);
    tracing::info!("Redis URL: {}", infra_config.redis.url);

    // Start server
    let addr = config.server.address().parse::<std::net::SocketAddr>()?;
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}

/// Initialize tracing with structured logging
fn tracing_init() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|| EnvFilter::new("info,identity=debug,sqlx=warn"));

    let subscriber = tracing_subscriber::registry().with(env_filter).with(
        fmt::layer()
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(true)
            .json(),
    );

    subscriber.init();
}

/// Graceful shutdown handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C received, shutting down gracefully...");
        }
        _ = terminate => {
            tracing::info!("SIGTERM received, shutting down gracefully...");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use sqlx::PgPool;

//     #[tokio::test]
//     async fn test_config_loading() {
//         std::env::set_var("DATABASE_URL", "postgres://test:test@localhost:5432/test");
//         std::env::set_var("REDIS_URL", "redis://localhost:6379");
//         std::env::set_var("JWT_SECRET", "test-secret-key-for-testing-only");

//         let config = Config::from_env();
//         assert_eq!(
//             config.database_url,
//             "postgres://test:test@localhost:5432/test"
//         );
//         assert_eq!(config.redis_url, "redis://localhost:6379");
//         assert_eq!(config.jwt_secret, "test-secret-key-for-testing-only");
//     }
// }
