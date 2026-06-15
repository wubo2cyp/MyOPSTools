//! agent-backend entry point.
//!
//! Wires up configuration, logging, database, router, and starts the axum server.

use agent_backend::{config::Config, db, router, state::AppState};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present (best effort).
    let _ = dotenvy::dotenv();

    init_tracing();

    let cfg = Config::from_env()?;
    info!(addr = %cfg.bind_addr, "starting agent-backend");

    let pool = db::init_pool(&cfg.database_url).await?;
    db::run_migrations(&pool).await?;

    let state = AppState::new(cfg.clone(), pool);
    let app = router::build(state).layer(cors_layer(&cfg)).layer(TraceLayer::new_for_http());

    let addr: SocketAddr = cfg.bind_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(%addr, "listening");
    axum::serve(listener, app).await?;
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();
}

fn cors_layer(_cfg: &Config) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
