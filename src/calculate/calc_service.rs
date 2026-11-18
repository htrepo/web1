use crate::calculate::db::{get_data, get_data_by_product, get_distinct_products};
use crate::calculate::error::CalcError;
use crate::common::model::sales::Sales;
use futures::future::join_all;
use tokio::task::JoinHandle;

pub async fn calc1() -> Result<f64, CalcError> {

    let data:Vec<Sales>=get_data().await?;
    // Multiply price * qty for each record, accumulate total
    let sum: f64 = data.iter().fold(0f64, |acc, item| {
        acc + (item.price * (item.quantity as f64))
    });

    Ok(sum)
}

/// Fetches data concurrently per product, then sums price*qty in parallel using rayon.
/// 1. get_distinct_products() → list of product names
/// 2. One tokio task per product → get_data_by_product() runs concurrently
/// 3. join_all waits for all tasks; rayon par_iter folds each chunk in parallel
pub async fn calc1parallel() -> Result<f64, CalcError> {

    // Step 1: get distinct product names (one lightweight query)
    let products: Vec<String> = get_distinct_products().await?;

    // Step 2: spawn one async task per product — all fetch concurrently
    let handles: Vec<JoinHandle<Result<f64, CalcError>>> = products
        .into_iter()
        .map(|product| {
            tokio::spawn(async move {
                let rows: Vec<Sales> = get_data_by_product(&product).await?;
                // Per-product subtotal
                let subtotal: f64 = rows.iter().fold(0f64, |acc, item| {
                    acc + (item.price * (item.quantity as f64))
                });
                Ok::<f64, CalcError>(subtotal)
            })
        })
        .collect();

    // Step 3: await all tasks and accumulate
    let results = join_all(handles).await;

    let mut total = 0f64;
    for result in results {
        // JoinError (task panic) wrapped as CalcError
        let subtotal = result
            .map_err(|e| CalcError::InvalidData(format!("task panicked: {e}")))?
            ?; // propagate CalcError from inside the task
        total += subtotal;
    }

    Ok(total)
}
