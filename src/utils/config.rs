use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_token: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let discord_token = env::var("DISCORD_TOKEN")
            .context("DISCORD_TOKEN must be set in environment")?;

        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL must be set in environment")?;

        Ok(Self {
            discord_token,
            database_url,
        })
    }
}
