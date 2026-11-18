use arrow_schema::Schema;
use crate::common::config::config::Config;
use crate::common::model::sales::Sales;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::sync::Arc;
use arrow::array::{Float64Array, Int32Array, Int64Array, StringArray, TimestampMillisecondArray};
use chrono::{Duration, Utc};
use rand::prelude::ThreadRng;
use rand::Rng;
use arrow::record_batch::RecordBatch;

pub fn write_parquet(config: &Config) {
    let schema: Arc<Schema> = Sales::schema();
    let file: File = File::create(&config.parquet_file_name).unwrap();
    tracing::info!("parquet file created: {:?}", file);
    let props: WriterProperties = WriterProperties::builder().build();
    let mut writer: ArrowWriter<File> =
        ArrowWriter::try_new(file, Arc::clone(&schema), Some(props)).unwrap();
    write_batches(&mut writer, &schema);
    writer.close().unwrap();
}

fn write_batches(writer: &mut ArrowWriter<File>, schema: &Arc<Schema>) {
    let products: [&str; 8] = ["Laptop", "Phone", "Tablet", "Monitor", "Keyboard",
        "Mouse", "Headphones", "Camera"];
    let mut rng: ThreadRng = rand::rng();
    let now_ms: i64 = Utc::now().timestamp_millis();
    let two_years_ms = Duration::days(730).num_milliseconds();

    let total_rows: i64 = 10_000_000;
    let batch_size: i64 = 100_000;
    let mut rows_done: i64 = 0i64;
    let mut id: i64 = 1i64;

    while rows_done < total_rows {
        let chunk: i64 = batch_size.min(total_rows - rows_done);

        let id_chunk: i64 = id + chunk;

        let ids: Vec<i64> = (id..id_chunk).collect::<Vec<_>>();

        let prods: Vec<&str> = (0..chunk)
            .map(|_| {
                products[rng.random_range(0..products.len())]
            })
            .collect::<Vec<_>>();

        let qtys: Vec<i32> = (0..chunk)
            .map(|_| rng.random_range(1i32..=100))
            .collect::<Vec<i32>>();

        let prices = (0..chunk)
            .map(|_| rng.random_range(100u32..=99999u32) as f64 / 100.0)
            .collect::<Vec<f64>>();

        let dates = (0..chunk)
            .map(|_| now_ms - rng.random_range(0..two_years_ms))
            .collect::<Vec<i64>>();

        let batch = RecordBatch::try_new(
            Arc::clone(schema),
            vec![
                Arc::new(Int64Array::from(ids)),
                Arc::new(StringArray::from(prods)),
                Arc::new(Int32Array::from(qtys)),
                Arc::new(Float64Array::from(prices)),
                Arc::new(TimestampMillisecondArray::from(dates)),
            ],
        ).unwrap();

        tracing::info!("write_batches. chunk: {}", chunk);
        writer.write(&batch).unwrap();
        id += chunk;
        rows_done += chunk;
    }
    tracing::info!("rows_done: {}", rows_done);
}
