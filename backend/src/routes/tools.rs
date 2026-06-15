//! GET /api/tools

use crate::state::AppState;
use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ToolDto {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub async fn list(State(state): State<AppState>) -> Json<Vec<ToolDto>> {
    let defs = state.tools.definitions();
    Json(
        defs.into_iter()
            .map(|t| ToolDto {
                name: t.name,
                description: t.description,
                input_schema: t.input_schema,
            })
            .collect(),
    )
}
