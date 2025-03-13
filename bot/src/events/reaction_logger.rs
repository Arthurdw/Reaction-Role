use poise::serenity_prelude::{Context, EventHandler, Reaction, async_trait};
use tracing::{info, instrument};

pub struct ReactionLogger;

#[async_trait]
impl EventHandler for ReactionLogger {
    #[instrument(skip(self, _ctx))]
    async fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        info!("Reaction added: {:?}", reaction);
    }
}
