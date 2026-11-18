use std::fs::File;
use arrow::array::{Float64Array, Int32Array, Int64Array, StringArray, TimestampMillisecondArray};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use sqlx::{Connection, PgConnection};
use crate::common::config::config::Config;
use arrow::record_batch::RecordBatch;


pub async fn load_db() -> Result<u64, sqlx::Error> {
    let config: Config = Config::default();
    let file_to_load = &config.parquet_file_name;
    let db_url = config.db_url();
    let mut conn=PgConnection::connect(&db_url).await?;

    let mut copy = conn.copy_in_raw(
        "COPY sales (id, product, quantity, price, sale_date) FROM STDIN (FORMAT CSV)"
    ).await?;

    let file = File::open(file_to_load).expect("parquet file not found");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    let reader = builder.with_batch_size(100_000).build().unwrap();
    let mut total: u64 = 0;

    for batch_result in reader {
        let batch = batch_result.unwrap();
        let csv = batch_to_csv(&batch);
        copy.send(csv.as_bytes()).await?;
        total += batch.num_rows() as u64;
        tracing::debug!("Sent {} rows, total so far: {}", batch.num_rows(), total);
    }

    copy.finish().await?;
    tracing::info!("Loaded {} rows into Postgres via COPY", total);
    Ok(total)
}

fn batch_to_csv(batch: &RecordBatch) -> String {
    let ids    = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
    let prods  = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
    let qtys   = batch.column(2).as_any().downcast_ref::<Int32Array>().unwrap();
    let prices = batch.column(3).as_any().downcast_ref::<Float64Array>().unwrap();
    let dates  = batch.column(4).as_any().downcast_ref::<TimestampMillisecondArray>().unwrap();

    let mut csv = String::with_capacity(batch.num_rows() * 64);
    for i in 0..batch.num_rows() {
        csv.push_str(&format!(
            "{},{},{},{:.2},{}\n",
            ids.value(i),
            prods.value(i),
            qtys.value(i),
            prices.value(i),
            dates.value(i)  // raw epoch millis — matches sale_date BIGINT
        ));
    }
    csv
}
