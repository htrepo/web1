use crate::load_db::db_loader_service;
use axum::http::StatusCode;

/// Handler that triggers the DB load process and returns HTTP 200 on success.
pub async fn load_db_handler() -> (StatusCode, &'static str) {
    match db_loader_service::load_db().await {
        Ok(_) => (StatusCode::OK, "loaded"),
        Err(e) => {
            tracing::error!("load_db failed: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "failed to load db")
        }
    }
}
