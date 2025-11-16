CREATE TABLE IF NOT EXISTS translation_channels (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL UNIQUE,
    webhook_url TEXT NOT NULL,
    source_language VARCHAR(10) NOT NULL DEFAULT 'zh',
    target_language VARCHAR(10) NOT NULL DEFAULT 'en',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS translation_history (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    original_message TEXT NOT NULL,
    translated_message TEXT NOT NULL,
    source_language VARCHAR(10) NOT NULL,
    target_language VARCHAR(10) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS guild_settings (
    guild_id BIGINT PRIMARY KEY,
    prefix VARCHAR(10) NOT NULL DEFAULT ';',
    default_source_lang VARCHAR(10) NOT NULL DEFAULT 'zh',
    default_target_lang VARCHAR(10) NOT NULL DEFAULT 'en',
    auto_translate BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_translation_channels_guild ON translation_channels(guild_id);
CREATE INDEX idx_translation_channels_active ON translation_channels(is_active);
CREATE INDEX idx_translation_history_channel ON translation_history(channel_id);
CREATE INDEX idx_translation_history_created ON translation_history(created_at);
CREATE INDEX idx_guild_settings_guild ON guild_settings(guild_id);
