use std::sync::Arc;

use poise::serenity_prelude::{Context, EventHandler, Ready, async_trait};
use tracing::info;

use super::BaseHandler;

pub struct PresenceHandler {
    base: Arc<BaseHandler>,
}

impl PresenceHandler {
    pub fn new(base: Arc<BaseHandler>) -> Self {
        info!("Enabled presence handler");
        Self { base }
    }
}

#[async_trait]
impl EventHandler for PresenceHandler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        ctx.set_presence(
            self.base.config.bot.get_activity(),
            self.base.config.bot.get_status(),
        );
    }
}
