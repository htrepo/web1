use crate::calculate::calc_controller::{calc_parallel_sum_handler, calc_sum_handler};
use crate::create_data::create_parquet_controller::generate_parquet_handler;
use crate::load_db::db_loader_controller::load_db_handler;
use axum::Router;
use axum::routing::post;

pub fn router() -> Router {
    Router::new()
        .route("/generate", post(generate_parquet_handler))
        .route("/load", post(load_db_handler))
        .route("/calc1", post(calc_sum_handler))
        .route("/calcp", post(calc_parallel_sum_handler))
}
