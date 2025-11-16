use crate::commands::{HelpCommand, SetLogCommand, StatsCommand};
use crate::db::{ChannelRepository, DatabasePool, HistoryRepository};
use crate::models::TranslationHistory;
use crate::services::TranslationService;
use anyhow::Result;
use serenity::all::{Context, EventHandler, Message, Ready};
use std::sync::Arc;

pub struct MessageHandler {
    db: DatabasePool,
    translation_service: Arc<TranslationService>,
}

impl MessageHandler {
    pub fn new(db: DatabasePool, translation_service: Arc<TranslationService>) -> Self {
        Self {
            db,
            translation_service,
        }
    }

    async fn handle_command(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let content = msg.content.trim();

        if !content.starts_with(';') {
            return Ok(());
        }

        let parts: Vec<&str> = content[1..].split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match command.as_str() {
            "help" | "h" => {
                HelpCommand::execute(ctx, msg).await?;
            }
            "set-log" => {
                SetLogCommand::execute(ctx, msg, args.to_vec(), &self.db).await?;
            }
            "remove-log" => {
                SetLogCommand::remove_log(ctx, msg, args.to_vec(), &self.db).await?;
            }
            "list-logs" => {
                SetLogCommand::list_logs(ctx, msg, &self.db).await?;
            }
            "translate" => {
                self.handle_manual_translation(ctx, msg, args).await?;
            }
            "languages" | "langs" => {
                self.show_languages(ctx, msg).await?;
            }
            "stats" => {
                StatsCommand::execute(ctx, msg, &self.db).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn handle_manual_translation(&self, ctx: &Context, msg: &Message, args: &[&str]) -> Result<()> {
        if args.len() < 3 {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Usage: `;translate <source-lang> <target-lang> <text>`\nExample: `;translate zh en 你好世界`",
                )
                .await?;
            return Ok(());
        }

        let source_lang = args[0];
        let target_lang = args[1];
        let text = args[2..].join(" ");

        let typing = msg.channel_id.start_typing(&ctx.http);

        match self
            .translation_service
            .translate_with_fallback(&text, source_lang, target_lang)
            .await
        {
            Ok(translated) => {
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!("**Translation ({} → {}):**\n{}", source_lang, target_lang, translated),
                    )
                    .await?;
            }
            Err(e) => {
                msg.channel_id
                    .say(&ctx.http, format!("❌ Translation failed: {}", e))
                    .await?;
            }
        }

        typing.stop();

        Ok(())
    }

    async fn show_languages(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let langs = r#"
**Supported Languages:**

**Chinese:**
• `zh`, `zh-CN`, `zh-Hans` - Simplified Chinese
• `zh-TW`, `zh-Hant` - Traditional Chinese

**Other Languages:**
• `en` - English
• `ja` - Japanese
• `ko` - Korean
• `de` - German
• `fr` - French
• `es` - Spanish
• `it` - Italian
• `pt` - Portuguese
• `ru` - Russian
• `nl` - Dutch
• `pl` - Polish
        "#;

        msg.channel_id.say(&ctx.http, langs).await?;

        Ok(())
    }

    async fn handle_auto_translation(&self, ctx: &Context, msg: &Message) -> Result<()> {
        if msg.author.bot {
            return Ok(());
        }

        let channel_config =
            ChannelRepository::get_by_channel_id(self.db.pool(), msg.channel_id.get() as i64).await?;

        if let Some(config) = channel_config {
            let typing = msg.channel_id.start_typing(&ctx.http);

            match self
                .translation_service
                .translate_with_fallback(
                    &msg.content,
                    &config.source_language,
                    &config.target_language,
                )
                .await
            {
                Ok(translated) => {
                    let username = if let Some(discrim) = msg.author.discriminator {
                        format!("{}#{}", msg.author.name, discrim)
                    } else {
                        msg.author.name.clone()
                    };
                    let user_id = msg.author.id.get();

                    let webhook_message = format!(
                        "{} (ID: {}) sent this:\n{}\n\nWhich translates to this:\n{}",
                        username, user_id, msg.content, translated
                    );

                    self.send_to_webhook(&config.webhook_url, &webhook_message).await?;

                    if let Some(guild_id) = msg.guild_id {
                        let history = TranslationHistory::new(
                            guild_id.get() as i64,
                            msg.channel_id.get() as i64,
                            msg.author.id.get() as i64,
                            msg.content.clone(),
                            translated,
                            config.source_language.clone(),
                            config.target_language.clone(),
                        );

                        HistoryRepository::create(self.db.pool(), &history).await.ok();
                    }
                }
                Err(e) => {
                    tracing::error!("Translation failed for channel {}: {}", msg.channel_id, e);
                }
            }

            typing.stop();
        }

        Ok(())
    }

    async fn send_to_webhook(&self, webhook_url: &str, content: &str) -> Result<()> {
        let client = reqwest::Client::new();

        let payload = serde_json::json!({
            "content": content,
            "username": "MegaChinese Translation",
        });

        client
            .post(webhook_url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if let Err(e) = self.handle_command(&ctx, &msg).await {
            tracing::error!("Command error: {}", e);
        }

        if !msg.content.starts_with(';') {
            if let Err(e) = self.handle_auto_translation(&ctx, &msg).await {
                tracing::error!("Auto-translation error: {}", e);
            }
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("Bot connected as {}", ready.user.name);
        tracing::info!("Ready to translate!");
    }
}
