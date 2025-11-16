use crate::models::{GuildSettings, TranslationChannel, TranslationHistory};
use anyhow::Result;
use sqlx::PgPool;

pub struct ChannelRepository;

impl ChannelRepository {
    pub async fn create(pool: &PgPool, channel: &TranslationChannel) -> Result<TranslationChannel> {
        let result = sqlx::query_as::<_, TranslationChannel>(
            r#"
            INSERT INTO translation_channels
            (guild_id, channel_id, webhook_url, source_language, target_language, is_active)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (channel_id)
            DO UPDATE SET
                webhook_url = EXCLUDED.webhook_url,
                source_language = EXCLUDED.source_language,
                target_language = EXCLUDED.target_language,
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(channel.guild_id)
        .bind(channel.channel_id)
        .bind(&channel.webhook_url)
        .bind(&channel.source_language)
        .bind(&channel.target_language)
        .bind(channel.is_active)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_channel_id(pool: &PgPool, channel_id: i64) -> Result<Option<TranslationChannel>> {
        let result = sqlx::query_as::<_, TranslationChannel>(
            "SELECT * FROM translation_channels WHERE channel_id = $1 AND is_active = true",
        )
        .bind(channel_id)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_guild(pool: &PgPool, guild_id: i64) -> Result<Vec<TranslationChannel>> {
        let results = sqlx::query_as::<_, TranslationChannel>(
            "SELECT * FROM translation_channels WHERE guild_id = $1 AND is_active = true ORDER BY created_at DESC",
        )
        .bind(guild_id)
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn delete(pool: &PgPool, channel_id: i64) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE translation_channels SET is_active = false, updated_at = NOW() WHERE channel_id = $1",
        )
        .bind(channel_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

pub struct GuildRepository;

#[allow(dead_code)]
impl GuildRepository {
    pub async fn get_or_create(pool: &PgPool, guild_id: i64) -> Result<GuildSettings> {
        let result = sqlx::query_as::<_, GuildSettings>(
            r#"
            INSERT INTO guild_settings (guild_id)
            VALUES ($1)
            ON CONFLICT (guild_id) DO UPDATE SET guild_id = EXCLUDED.guild_id
            RETURNING *
            "#,
        )
        .bind(guild_id)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn update_prefix(pool: &PgPool, guild_id: i64, prefix: &str) -> Result<()> {
        sqlx::query("UPDATE guild_settings SET prefix = $1, updated_at = NOW() WHERE guild_id = $2")
            .bind(prefix)
            .bind(guild_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_languages(
        pool: &PgPool,
        guild_id: i64,
        source: &str,
        target: &str,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE guild_settings SET default_source_lang = $1, default_target_lang = $2, updated_at = NOW() WHERE guild_id = $3",
        )
        .bind(source)
        .bind(target)
        .bind(guild_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct HistoryRepository;

#[allow(dead_code)]
impl HistoryRepository {
    pub async fn create(pool: &PgPool, history: &TranslationHistory) -> Result<TranslationHistory> {
        let result = sqlx::query_as::<_, TranslationHistory>(
            r#"
            INSERT INTO translation_history
            (guild_id, channel_id, user_id, original_message, translated_message, source_language, target_language)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(history.guild_id)
        .bind(history.channel_id)
        .bind(history.user_id)
        .bind(&history.original_message)
        .bind(&history.translated_message)
        .bind(&history.source_language)
        .bind(&history.target_language)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_recent(
        pool: &PgPool,
        channel_id: i64,
        limit: i64,
    ) -> Result<Vec<TranslationHistory>> {
        let results = sqlx::query_as::<_, TranslationHistory>(
            "SELECT * FROM translation_history WHERE channel_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(channel_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(results)
    }
}
