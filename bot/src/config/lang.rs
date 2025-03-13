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

#[derive(Debug, Serialize, Deserialize)]
pub struct Lang {
    pub reaction_logger: ReactionLogger,
}

pub fn load() -> Result<Lang> {
    load_yaml_config("config/lang.yaml")
}
