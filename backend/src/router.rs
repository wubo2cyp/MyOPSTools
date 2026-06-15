//! Axum router assembly.

use crate::routes;
use crate::state::AppState;
use axum::{routing::get, Json, Router};
use serde_json::json;

pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(routes::health::health))
        .nest("/api", api_routes())
        .fallback(not_found)
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/sessions", get(routes::sessions::list).post(routes::sessions::create))
        .route(
            "/sessions/:id",
            get(routes::sessions::get)
                .put(routes::sessions::update)
                .delete(routes::sessions::delete),
        )
        .route("/sessions/:id/messages", get(routes::messages::list).post(routes::messages::create))
        .route("/sessions/:id/runs", axum::routing::post(routes::runs::create))
        .route(
            "/sessions/:id/runs/:run_id/cancel",
            axum::routing::post(routes::runs::cancel),
        )
        .route("/tools", get(routes::tools::list))
}

async fn not_found() -> Json<serde_json::Value> {
    Json(json!({ "error": { "code": "not_found", "message": "route not found" } }))
}
