use serenity::all::{Context, Message};
use anyhow::Result;

pub struct HelpCommand;

impl HelpCommand {
    pub async fn execute(ctx: &Context, msg: &Message) -> Result<()> {
        let help_text = r#"
**MegaChinese Translation Bot** 🇨🇳

**Translation Features:**
• Automatic translation of messages in configured channels
• Support for multiple Chinese dialects and languages
• Multi-provider fallback (Google Translate, DeepL)

**Commands:**

`;set-log <language> <channel-id> <webhook-url>`
Setup translation logging for a channel
Example: `;set-log chinese #translations https://discord.com/api/webhooks/...`

`;remove-log <channel-id>`
Remove translation logging from a channel

`;list-logs`
List all configured translation channels in this server

`;translate <source-lang> <target-lang> <text>`
Manually translate text
Example: `;translate zh en 你好世界`

`;languages`
Show all supported languages

`;stats`
Show translation statistics for this server

**Supported Languages:**
• Chinese (Simplified): `zh`, `zh-CN`, `zh-Hans`
• Chinese (Traditional): `zh-TW`, `zh-Hant`
• English: `en`
• Japanese: `ja`
• Korean: `ko`
• German: `de`
• French: `fr`
• Spanish: `es`
• Italian: `it`
• Portuguese: `pt`
• Russian: `ru`

**How Translation Logging Works:**
When you set up a translation channel, any message sent in that channel will be automatically translated and logged to the webhook URL with the format:

```
<@user_id> username#tag sent this:
Original message here

Which translates to this:
Translated message here
```

**Note:** The bot requires appropriate permissions to read messages in the configured channels.
        "#;

        msg.channel_id.say(&ctx.http, help_text).await?;

        Ok(())
    }
}
