use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::utils::io::load_yaml_config;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BotConfig {
    pub updater: Updater,
    pub console: Console,
    pub bot: Bot,
    pub reaction_logging: ReactionLogging,
    pub token: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Updater {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Console {
    pub colors: bool,
    pub print_imports: bool,
    pub print_log: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub prefix: String,
    pub rich_presence_enabled: bool,
    pub rich_presence: String,
    pub rich_presence_type: String,
    pub bot_status: String,
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

pub(crate) fn load() -> Result<BotConfig> {
    let config = load_yaml_config::<BotConfig>("./config/config.yaml")?;
    config.bot.validate()?;
    Ok(config)
}
