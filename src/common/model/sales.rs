use arrow_schema::{DataType, Field, Schema, TimeUnit};
use std::sync::Arc;

/// Domain model for a sales record.
/// Types aligned with Postgres (i64=BIGINT, i32=INT) and Arrow (Int64, Int32).
#[derive(Debug, sqlx::FromRow)]
pub struct Sales {
    pub id: i64,
    pub product: String,
    pub quantity: i32,
    pub price: f64,
    pub sale_date: i64, // epoch milliseconds
}

// Arc = Atomically Reference Counted — Rust's thread-safe shared pointer.

impl Sales {
    /// Returns the Arrow schema for the sales parquet file.
    /// Centralised here so schema stays in sync with the struct.
    pub fn schema() -> Arc<Schema> {
        let sch: Schema = Schema::new(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("product", DataType::Utf8, false),
            Field::new("quantity", DataType::Int32, false),
            Field::new("price", DataType::Float64, false),
            Field::new(
                "sale_date",
                DataType::Timestamp(TimeUnit::Millisecond, None),
                false,
            ),
        ]);

        Arc::new(sch)
    }
}
