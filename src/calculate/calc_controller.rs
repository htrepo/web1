use crate::calculate::calc_service::{calc1, calc1parallel};
use axum::http::StatusCode;

pub async fn calc_sum_handler() -> (StatusCode, String) {
    let result = calc1().await.unwrap();
    (StatusCode::OK, format!("{:.2}", result))
}

pub async fn calc_parallel_sum_handler() -> (StatusCode, String) {
    let result = calc1parallel().await.unwrap();
    (StatusCode::OK, format!("{:.2}", result))
}
