use mongodb::{error::Result, Client, Database};
use serenity::prelude::*;

/// Manages the database connection and access
pub struct DbClient {
    db_client: Client,
    db_name: String,
}

impl DbClient {
    /// Constructs a new database manager and connects it to the specified database
    pub async fn new(db_uri: &str, db_name: &str) -> Result<Self> {
        Ok(Self {
            db_client: Client::with_uri_str(db_uri).await?,
            db_name: db_name.into(),
        })
    }

    pub fn database(&self) -> Database {
        self.db_client.database(&self.db_name)
    }
}

impl TypeMapKey for DbClient {
    type Value = Self;
}
