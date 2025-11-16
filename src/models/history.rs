use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TranslationHistory {
    pub id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub user_id: i64,
    pub original_message: String,
    pub translated_message: String,
    pub source_language: String,
    pub target_language: String,
    pub created_at: DateTime<Utc>,
}

impl TranslationHistory {
    pub fn new(
        guild_id: i64,
        channel_id: i64,
        user_id: i64,
        original_message: String,
        translated_message: String,
        source_language: String,
        target_language: String,
    ) -> Self {
        Self {
            id: 0,
            guild_id,
            channel_id,
            user_id,
            original_message,
            translated_message,
            source_language,
            target_language,
            created_at: Utc::now(),
        }
    }
}
