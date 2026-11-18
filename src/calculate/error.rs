#[derive(Debug)]

pub enum CalcError {
    DbError(sqlx::Error),
    InvalidData(String),
    // add more variants as needed
}

// lets sqlx::Error convert automatically via ?
impl From<sqlx::Error> for CalcError {
    fn from(e: sqlx::Error) -> Self {
        CalcError::DbError(e)
    }
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::DbError(e) => write!(f, "Database error: {e}"),
            CalcError::InvalidData(msg) => write!(f, "Invalid data: {msg}"),
        }
    }
}
