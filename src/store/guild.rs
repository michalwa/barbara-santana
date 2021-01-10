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
            prefix: "%".into(),
        }
    }
}

impl StoreItem for GuildSettings {
    type Key = GuildId;
}

impl StoreItemKey for GuildId {
    type DocumentId = u64;
    fn doc_id(&self) -> Self::DocumentId { self.0 }
}
