pub struct Config {
    pub db_host:            String,
    pub db_port:            u16,
    pub db_user:            String,
    pub db_password:        String,
    pub db_name:            String,
    pub parquet_file_name:  String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db_host:           "localhost".to_string(),
            db_port:           5432,
            db_user:           "sa".to_string(),
            db_password:       "Password1!".to_string(),
            db_name:           "rusty".to_string(),
            parquet_file_name: "sales.parquet".to_string(),
        }
    }
}

impl Config {
    /// Builds a Postgres connection URL from config fields.
    pub fn db_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
                self.db_user, self.db_password, self.db_host, self.db_port, self.db_name)
    }
}
