use anyhow::{anyhow, Result};
use serenity::all::{Context, GuildId, Message, UserId};

pub struct PermissionChecker;

impl PermissionChecker {
    pub async fn is_admin(ctx: &Context, msg: &Message) -> Result<bool> {
        let guild_id = msg
            .guild_id
            .ok_or_else(|| anyhow!("This command can only be used in a server"))?;

        let member = guild_id.member(&ctx.http, msg.author.id).await?;

        if Self::is_guild_owner(ctx, guild_id, msg.author.id).await? {
            return Ok(true);
        }

        let guild = guild_id.to_partial_guild(&ctx.http).await?;
        let permissions = guild.member_permissions(&member);

        Ok(permissions.administrator()
            || permissions.manage_guild()
            || permissions.manage_channels())
    }

    async fn is_guild_owner(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Result<bool> {
        let guild = guild_id.to_partial_guild(&ctx.http).await?;
        Ok(guild.owner_id == user_id)
    }

    pub async fn require_admin(ctx: &Context, msg: &Message) -> Result<()> {
        if !Self::is_admin(ctx, msg).await? {
            msg.channel_id
                .say(
                    &ctx.http,
                    "❌ You need Administrator, Manage Server, or Manage Channels permission to use this command.",
                )
                .await?;
            return Err(anyhow!("Insufficient permissions"));
        }
        Ok(())
    }
}
