use std::collections::HashMap;

use anyhow::{Result, bail};
use poise::serenity_prelude::{ActivityData, OnlineStatus};
use serde::{Deserialize, Serialize};

use crate::utils::io::load_yaml_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub console: Console,
    pub bot: Bot,
    pub reaction_logging: ReactionLogging,
    pub token: Token,
    pub leveling: Leveling,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Console {
    pub level: String,
}

impl Console {
    pub fn validate(&self) -> Result<()> {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.level.as_str()) {
            bail!(
                "Invalid log level: {}. Valid values are: {:?}",
                self.level,
                valid_levels
            );
        }
        Ok(())
    }

    pub fn tracing_level(&self) -> tracing::Level {
        match self.level.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub rich_presence_enabled: bool,
    pub rich_presence: String,
    pub rich_presence_type: String,
    pub bot_status: String,
    pub prefix: String,
    pub prefix_enabled: bool,
}

impl Bot {
    pub fn validate(&self) -> Result<()> {
        let valid_rich_presence_types = ["playing", "watching", "listening to", "streaming"];
        if !valid_rich_presence_types.contains(&self.rich_presence_type.as_str()) {
            bail!(
                "Invalid rich_presence_type: {}. Valid values are: {:?}",
                self.rich_presence_type,
                valid_rich_presence_types
            );
        }

        let valid_bot_statuses = ["online", "dnd", "idle", "invisible"];
        if !valid_bot_statuses.contains(&self.bot_status.as_str()) {
            bail!(
                "Invalid bot_status: {}. Valid values are: {:?}",
                self.bot_status,
                valid_bot_statuses
            );
        }

        Ok(())
    }

    pub fn get_activity(&self) -> Option<ActivityData> {
        if self.rich_presence_enabled {
            match self.rich_presence_type.as_str() {
                "playing" => Some(ActivityData::playing(self.rich_presence.clone())),
                "watching" => Some(ActivityData::watching(self.rich_presence.clone())),
                "listening to" => Some(ActivityData::listening(self.rich_presence.clone())),
                "streaming" => Some(
                    ActivityData::streaming(
                        self.rich_presence.clone(),
                        "https://www.twitch.tv/arthurdw_",
                    )
                    .unwrap(),
                ),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_status(&self) -> OnlineStatus {
        match self.bot_status.as_str() {
            "dnd" => OnlineStatus::DoNotDisturb,
            "idle" => OnlineStatus::Idle,
            "invisible" => OnlineStatus::Invisible,
            _ => OnlineStatus::Online,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionLogging {
    pub enabled: bool,
    pub log_guild: u64,
    pub log_channel: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token_env_enabled: bool,
    pub token_env: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leveling {
    pub enabled: bool,
    pub max_top: u64,
    pub guild: u64,
    pub notifications_channel: u64,
    pub roles: HashMap<u64, u64>,
}

pub fn load() -> Result<BotConfig> {
    let config = load_yaml_config::<BotConfig>("./config/config.yaml")?;
    config.console.validate()?;
    config.bot.validate()?;
    Ok(config)
}
