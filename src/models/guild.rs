use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GuildSettings {
    pub guild_id: i64,
    pub prefix: String,
    pub default_source_lang: String,
    pub default_target_lang: String,
    pub auto_translate: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for GuildSettings {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            guild_id: 0,
            prefix: ";".to_string(),
            default_source_lang: "zh".to_string(),
            default_target_lang: "en".to_string(),
            auto_translate: true,
            created_at: now,
            updated_at: now,
        }
    }
}
