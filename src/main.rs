mod commands;
mod db;
mod handlers;
mod models;
mod services;
mod utils;

use anyhow::Result;
use db::DatabasePool;
use handlers::MessageHandler;
use serenity::all::{Client, GatewayIntents};
use services::{LibreTranslateProvider, LingvaProvider, MyMemoryProvider, TranslationService};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::Config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mega_chinese=info,serenity=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;

    tracing::info!("Initializing MegaChinese Translation Bot...");

    let db = DatabasePool::new(&config.database_url).await?;
    tracing::info!("Database connected and migrated");

    db.health_check().await?;
    tracing::info!("Database health check passed");

    let mut translation_service = TranslationService::new();

    let libre_provider = Arc::new(LibreTranslateProvider::new());
    translation_service.add_provider(libre_provider);
    tracing::info!("LibreTranslate provider configured (primary)");

    let mymemory_provider = Arc::new(MyMemoryProvider::new());
    translation_service.add_provider(mymemory_provider);
    tracing::info!("MyMemory provider configured (fallback 1)");

    let lingva_provider = Arc::new(LingvaProvider::new());
    translation_service.add_provider(lingva_provider);
    tracing::info!("Lingva provider configured (fallback 2)");

    let translation_service = Arc::new(translation_service);

    let handler = MessageHandler::new(db, translation_service);

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(handler)
        .await?;

    tracing::info!("Starting Discord bot...");

    tokio::select! {
        result = client.start() => {
            if let Err(e) = result {
                tracing::error!("Client error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received shutdown signal");
        }
    }

    tracing::info!("Bot shutdown complete");

    Ok(())
}
