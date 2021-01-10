use serenity::prelude::*;
use serde::Deserialize;
use std::fs;
use std::error::Error;

use crate::db;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub bot: BotConfig,
    pub database: DbConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let string = fs::read_to_string("config.toml")?;
        Ok(toml::from_str(&string)?)
    }
}

impl TypeMapKey for AppConfig {
    type Value = Self;
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub default_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub uri: String,
    pub name: String,
}

impl DbConfig {
    pub async fn into_client(&self) -> db::Result<db::DbClient> {
        db::DbClient::new(&self.uri, &self.name).await
    }
}
