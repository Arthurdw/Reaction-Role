use poise::serenity_prelude::{Context, EventHandler, Ready, async_trait};
use tracing::info;

pub struct InfoHandler;

#[async_trait]
impl EventHandler for InfoHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        info!("Connected to {} guild(s)", ready.guilds.len());
        info!("Using {} shard(s)", ready.shard.map_or(1, |s| s.total));
        info!("Using Discord api version: v{}", ready.version);
    }
}
