use std::sync::Arc;

use crate::config::{BotConfig, lang::Lang};

pub mod reaction_logger;
pub mod reaction_roles;

pub(crate) struct BaseHandler {
    config: Arc<BotConfig>,
    lang: Arc<Lang>,
}

impl BaseHandler {
    pub(crate) fn new(config: Arc<BotConfig>, lang: Arc<Lang>) -> Self {
        Self { config, lang }
    }
}
