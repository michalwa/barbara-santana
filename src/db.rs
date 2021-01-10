pub use mongodb::error::Result;
use mongodb::sync::{Client, Database};
use serenity::prelude::*;

/// Manages the database connection and access
#[derive(Clone)]
pub struct DbClient {
    db_client: Client,
    db_name: String,
}

impl DbClient {
    /// Constructs a new database manager and connects it to the specified database
    pub async fn new(db_uri: &str, db_name: &str) -> Result<Self> {
        Ok(Self {
            db_client: Client::with_uri_str(db_uri)?,
            db_name: db_name.into(),
        })
    }

    /// Returns a handle to the main database
    pub fn database(&self) -> Database {
        self.db_client.database(&self.db_name)
    }
}

impl TypeMapKey for DbClient {
    type Value = Self;
}
