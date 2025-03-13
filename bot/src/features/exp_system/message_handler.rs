use anyhow::Result;
use poise::serenity_prelude::{Context, EventHandler, Message, async_trait};
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::features::BaseHandler;

use super::io::Database;

pub struct MessageHandler {
    base: Arc<BaseHandler>,
    db: Arc<Database>,
}

impl MessageHandler {
    pub fn new(base: Arc<BaseHandler>, db: Arc<Database>) -> Self {
        info!("Enabled leveling message handler");
        Self { base, db }
    }

    async fn on_message(&self, ctx: Context, message: Message) -> Result<()> {
        let user_id = message.author.id.get();
        let earned = rand::random::<u64>() % 6 + 1;
        debug!("User {} earned {} exp", user_id, earned);
        let exp = self.db.add_experience(user_id, earned as i64).await?;
        debug!("User {} now has {} exp", user_id, exp);

        // TODO: Check if user leveled up, and if so, send a message (if level is less than 50 only
        // send every 5 levels)
        // TODO: check on levelup if user is allowed a role

        Ok(())
    }
}

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }

        if let Err(e) = self.on_message(ctx, message).await {
            error!("Error handling message: {:?}", e);
        }
    }
}
