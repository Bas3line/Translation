use crate::db::DatabasePool;
use anyhow::Result;
use serenity::all::{Context, Message};

pub struct StatsCommand;

impl StatsCommand {
    pub async fn execute(ctx: &Context, msg: &Message, db: &DatabasePool) -> Result<()> {
        let guild_id = msg.guild_id.map(|g| g.get() as i64);

        if guild_id.is_none() {
            msg.channel_id
                .say(&ctx.http, "This command can only be used in a server.")
                .await?;
            return Ok(());
        }

        let guild_id = guild_id.unwrap();

        let channel_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM translation_channels WHERE guild_id = $1 AND is_active = true",
        )
        .bind(guild_id)
        .fetch_one(db.pool())
        .await?;

        let translation_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM translation_history WHERE guild_id = $1",
        )
        .bind(guild_id)
        .fetch_one(db.pool())
        .await?;

        let recent_translations: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM translation_history WHERE guild_id = $1 AND created_at > NOW() - INTERVAL '24 hours'",
        )
        .bind(guild_id)
        .fetch_one(db.pool())
        .await?;

        let stats_message = format!(
            r#"**Translation Statistics**

📊 **Server Stats:**
• Active translation channels: {}
• Total translations: {}
• Translations (24h): {}

**Translation Providers:**
• Primary: LibreTranslate (Free & Open Source)
• Fallback 1: MyMemory
• Fallback 2: Lingva

Use `;list-logs` to see configured channels."#,
            channel_count, translation_count, recent_translations
        );

        msg.channel_id.say(&ctx.http, stats_message).await?;

        Ok(())
    }
}
