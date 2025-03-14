use anyhow::Result;
use poise::serenity_prelude::{
    CacheHttp, ChannelId, Context, CreateEmbed, CreateMessage, EventHandler, Message, RoleId, User,
    async_trait,
};
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::{
    features::BaseHandler,
    utils::{
        color::random_color,
        formatting::{format_member, format_role},
    },
};

use super::{
    formatting::format_user_data,
    io::{Database, models::UserData},
    levels::convert_exp_to_level,
};

pub struct MessageHandler {
    base: Arc<BaseHandler>,
    db: Arc<Database>,
}

impl MessageHandler {
    pub fn new(base: Arc<BaseHandler>, db: Arc<Database>) -> Self {
        info!("Enabled leveling message handler");
        Self { base, db }
    }

    async fn build_formatter(
        &self,
        user: &User,
        data: &UserData,
        role: Option<&RoleId>,
    ) -> impl Fn(&str) -> String {
        move |message| {
            let mut message = message.to_string();
            format_member(&mut message, user);
            format_user_data(&mut message, data);

            if let Some(role) = role {
                format_role(&mut message, role);
            }

            message
        }
    }

    async fn send_levelup_message(
        &self,
        ctx: Context,
        user: &User,
        role: Option<&RoleId>,
    ) -> Result<()> {
        let cfg = &self.base.config.leveling;
        let lang = &self.base.lang.exp_system;
        let channel = ChannelId::new(cfg.notifications_channel);
        let data = self.db.get_user_data(user.id.get()).await?;
        let fmt = self.build_formatter(user, &data, role).await;

        let mut message = fmt(&lang.levelup_message);

        if role.is_some() {
            message.push_str(&fmt(&lang.levelup_message_role));
        }

        channel
            .send_message(
                &ctx.http(),
                CreateMessage::default().embed(
                    CreateEmbed::default()
                        .description(message)
                        .color(random_color()),
                ),
            )
            .await?;

        Ok(())
    }

    async fn levelup_handler(
        &self,
        ctx: Context,
        user: &User,
        before: i64,
        after: i64,
    ) -> Result<()> {
        debug!("User {} levelup check", user.id.get());
        let level_before = convert_exp_to_level(before);
        let level_after = convert_exp_to_level(after);

        debug!(
            "User {} was level {} before, now level {} (exp: {} -> {})",
            user.id.get(),
            level_before,
            level_after,
            before,
            after
        );

        if level_before >= level_after {
            debug!("User {} did not level up", user.id.get());
            return Ok(());
        }

        debug!("User {} leveled up", user.id.get());

        // Don't send a message for every level up, only every 5 levels if level is less than 30
        if level_after <= 30 && level_after % 5 != 0 {
            debug!("Hiding levelup message for user {}", user.id.get());
            return Ok(());
        }

        // TODO: check on levelup if user is allowed a role
        self.send_levelup_message(ctx, user, None).await?;

        Ok(())
    }

    async fn on_message(&self, ctx: Context, message: Message) -> Result<()> {
        let user_id = message.author.id.get();
        let earned = rand::random::<u64>() % 6 + 1;
        debug!("User {} earned {} exp", user_id, earned);
        let (before, after) = self.db.add_experience(user_id, earned as i64).await?;
        debug!("User {} now has {} exp", user_id, after);

        self.levelup_handler(ctx, &message.author, before, after)
            .await?;

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
