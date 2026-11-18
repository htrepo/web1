use crate::calculate::error::CalcError;
use crate::common::config::config::Config;
use crate::common::model::sales::Sales;
use sqlx::{Connection, PgConnection};

/// Connects to Postgres and fetches all sales rows.
pub async fn get_data() -> Result<Vec<Sales>, CalcError> {
    let config: Config = Config::default();
    let db_url = config.db_url();

    if db_url.is_empty() {
        return Err(CalcError::InvalidData("db_url is empty".into()));
    }

    let mut conn = PgConnection::connect(&db_url).await?;

    // Sales derives FromRow — sqlx maps each column directly to struct fields
    let sql = "SELECT id, product, quantity, price, sale_date FROM sales";
    let rows: Vec<Sales> = sqlx::query_as::<_, Sales>(sql).fetch_all(&mut conn).await?;

    Ok(rows)
}

/// Fetches the distinct product names from the sales table.
pub async fn get_distinct_products() -> Result<Vec<String>, CalcError> {
    let config: Config = Config::default();
    let db_url = config.db_url();

    if db_url.is_empty() {
        return Err(CalcError::InvalidData("db_url is empty".into()));
    }

    let mut conn = PgConnection::connect(&db_url).await?;

    // Returns one column "product" per distinct name
    let rows: Vec<(String,)> =
        sqlx::query_as("SELECT DISTINCT product FROM sales ORDER BY product")
            .fetch_all(&mut conn)
            .await?;

    Ok(rows.into_iter().map(|(p,)| p).collect())
}

/// Connects to Postgres and fetches sales rows filtered by product.
pub async fn get_data_by_product(product: &str) -> Result<Vec<Sales>, CalcError> {
    let config: Config = Config::default();
    let db_url = config.db_url();

    if db_url.is_empty() {
        return Err(CalcError::InvalidData("db_url is empty".into()));
    }

    let mut conn = PgConnection::connect(&db_url).await?;

    // $1 is a bind parameter — sqlx binds `product` safely (no SQL injection)
    let sql = "SELECT id, product, quantity, price, sale_date FROM sales WHERE product = $1";
    let rows: Vec<Sales> = sqlx::query_as::<_, Sales>(sql)
        .bind(product)
        .fetch_all(&mut conn)
        .await?;

    Ok(rows)
}
