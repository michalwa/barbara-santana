use serenity::model::id::GuildId;
use super::*;

/// Guild-scope bot configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct GuildSettings {
    pub prefix: String,
}

impl Default for GuildSettings {
    fn default() -> Self {
        Self {
            prefix: "%".to_owned(),
        }
    }
}

impl StoreItem for GuildSettings {
    type Key = GuildId;
}

impl DocId for GuildId {
    type Id = u64;
    fn doc_id(&self) -> Self::Id { self.0 }
}
