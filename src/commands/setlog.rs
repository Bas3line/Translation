use crate::db::{ChannelRepository, DatabasePool};
use crate::models::TranslationChannel;
use crate::utils::PermissionChecker;
use anyhow::{anyhow, Result};
use serenity::all::{Context, Message};

pub struct SetLogCommand;

impl SetLogCommand {
    pub async fn execute(
        ctx: &Context,
        msg: &Message,
        args: Vec<&str>,
        db: &DatabasePool,
    ) -> Result<()> {
        PermissionChecker::require_admin(ctx, msg).await?;

        if args.len() < 3 {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Usage: `;set-log <language> <channel-id> <webhook-url>`\nExample: `;set-log chinese #translations https://discord.com/api/webhooks/...`",
                )
                .await?;
            return Ok(());
        }

        let guild_id = msg
            .guild_id
            .ok_or_else(|| anyhow!("This command can only be used in a server"))?;

        let source_lang = Self::parse_language(args[0])?;
        let channel_id = Self::parse_channel_id(args[1])?;
        let webhook_url = args[2];

        if !Self::validate_webhook_url(webhook_url) {
            msg.channel_id
                .say(&ctx.http, "Invalid webhook URL. Please provide a valid Discord webhook URL.")
                .await?;
            return Ok(());
        }

        let translation_channel = TranslationChannel::new(
            guild_id.get() as i64,
            channel_id,
            webhook_url.to_string(),
            source_lang.clone(),
            "en".to_string(),
        );

        ChannelRepository::create(db.pool(), &translation_channel).await?;

        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "✅ Translation logging configured!\nChannel: <#{}>\nLanguage: {} → en\nWebhook: Set",
                    channel_id, source_lang
                ),
            )
            .await?;

        Ok(())
    }

    pub async fn remove_log(
        ctx: &Context,
        msg: &Message,
        args: Vec<&str>,
        db: &DatabasePool,
    ) -> Result<()> {
        PermissionChecker::require_admin(ctx, msg).await?;

        if args.is_empty() {
            msg.channel_id
                .say(&ctx.http, "Usage: `;remove-log <channel-id>`")
                .await?;
            return Ok(());
        }

        let channel_id = Self::parse_channel_id(args[0])?;

        let deleted = ChannelRepository::delete(db.pool(), channel_id).await?;

        if deleted {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("✅ Translation logging removed for <#{}>", channel_id),
                )
                .await?;
        } else {
            msg.channel_id
                .say(&ctx.http, "❌ No translation logging found for that channel")
                .await?;
        }

        Ok(())
    }

    pub async fn list_logs(ctx: &Context, msg: &Message, db: &DatabasePool) -> Result<()> {
        PermissionChecker::require_admin(ctx, msg).await?;

        let guild_id = msg
            .guild_id
            .ok_or_else(|| anyhow!("This command can only be used in a server"))?;

        let channels = ChannelRepository::get_by_guild(db.pool(), guild_id.get() as i64).await?;

        if channels.is_empty() {
            msg.channel_id
                .say(&ctx.http, "No translation channels configured for this server.")
                .await?;
            return Ok(());
        }

        let mut response = String::from("**Configured Translation Channels:**\n\n");
        for channel in channels {
            response.push_str(&format!(
                "• <#{}> - {} → {}\n",
                channel.channel_id, channel.source_language, channel.target_language
            ));
        }

        msg.channel_id.say(&ctx.http, response).await?;

        Ok(())
    }

    fn parse_language(lang: &str) -> Result<String> {
        let lang_lower = lang.to_lowercase();
        let normalized = match lang_lower.as_str() {
            "chinese" | "zh" | "cn" => "zh",
            "english" | "en" => "en",
            "japanese" | "ja" | "jp" => "ja",
            "korean" | "ko" | "kr" => "ko",
            "german" | "de" => "de",
            "french" | "fr" => "fr",
            "spanish" | "es" => "es",
            "italian" | "it" => "it",
            "portuguese" | "pt" => "pt",
            "russian" | "ru" => "ru",
            other => other,
        };
        Ok(normalized.to_string())
    }

    fn parse_channel_id(input: &str) -> Result<i64> {
        let cleaned = input.trim_start_matches("<#").trim_end_matches('>');
        cleaned
            .parse::<i64>()
            .map_err(|_| anyhow!("Invalid channel ID"))
    }

    fn validate_webhook_url(url: &str) -> bool {
        url.starts_with("https://discord.com/api/webhooks/")
            || url.starts_with("https://discordapp.com/api/webhooks/")
    }
}
