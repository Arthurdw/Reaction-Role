use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::utils::io::load_yaml_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionRolesConfig {
    reaction_roles: std::collections::HashMap<u64, Vec<(String, u64)>>,
}

pub fn load() -> Result<ReactionRolesConfig> {
    load_yaml_config("config/reaction_roles.yaml")
}
