use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::utils::io::load_yaml_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionLogger {
    pub added: ReactionEvent,
    pub removed: ReactionEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionEvent {
    pub title: String,
    pub content: String,
    pub footer: Footer,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Footer {
    pub text: String,
    pub icon: String,
    pub timestamp: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub random: bool,
    pub color: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpSystem {
    pub fetching: String,
    pub title: String,
    pub message: String,
    pub top_msg: String,
    pub top_title: String,
    pub top_no_users: String,
    pub top_line: String,
    pub levelup_message: String,
    pub levelup_message_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lang {
    pub reaction_logger: ReactionLogger,
    pub exp_system: ExpSystem,
}

pub fn load() -> Result<Lang> {
    load_yaml_config("config/lang.yaml")
}
