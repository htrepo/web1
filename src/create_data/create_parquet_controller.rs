use crate::common::config::config::Config;
use crate::create_data::create_parquet_service::write_parquet;
use axum::http::StatusCode;
use tokio::task;

pub async fn generate_parquet_handler() -> (StatusCode, &'static str) {
    let config: Config = Config::default();
    task::spawn_blocking(move || {
        write_parquet(&config);
    });
    tracing::info!("Parquet generation submitted");
    (StatusCode::ACCEPTED, "submitted")
}
