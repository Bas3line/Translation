use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TranslationChannel {
    pub id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub webhook_url: String,
    pub source_language: String,
    pub target_language: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TranslationChannel {
    pub fn new(
        guild_id: i64,
        channel_id: i64,
        webhook_url: String,
        source_language: String,
        target_language: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            guild_id,
            channel_id,
            webhook_url,
            source_language,
            target_language,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
