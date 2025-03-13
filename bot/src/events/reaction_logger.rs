use anyhow::{Result, bail};
use std::sync::Arc;

use poise::serenity_prelude::{
    ChannelId, Color, Context, CreateEmbed, CreateEmbedFooter, CreateMessage, EventHandler,
    GuildId, Member, PartialGuild, Reaction, ReactionType, async_trait,
};
use tracing::{debug, error, info};

use crate::{
    config::lang::ReactionEvent,
    utils::formatting::{format_channel_id, format_member, format_partial_guild, format_reaction},
};

use super::BaseHandler;

pub struct ReactionLogger {
    base: Arc<BaseHandler>,
}

impl ReactionLogger {
    pub fn new(base: Arc<BaseHandler>) -> Self {
        info!("Enabled reaction logging");
        Self { base }
    }
}

impl ReactionLogger {
    fn build_formatter(
        &self,
        member: &Member,
        guild: &PartialGuild,
        channel_id: &ChannelId,
        reaction: &ReactionType,
    ) -> impl Fn(&str) -> String {
        move |message| {
            let mut message = message.to_string();
            format_member(&mut message, member);
            format_partial_guild(&mut message, guild);
            format_channel_id(&mut message, channel_id);
            format_reaction(&mut message, reaction);
            message
        }
    }

    async fn get_member(
        &self,
        ctx: &Context,
        reaction: Reaction,
        guild_id: &GuildId,
    ) -> Result<Member> {
        match &reaction.member {
            Some(member) => Ok(member.clone()),
            None => match reaction.user_id {
                Some(user_id) => match guild_id.member(&ctx.http, user_id).await {
                    Ok(member) => Ok(member),
                    Err(e) => bail!("Error getting member: {:?}", e),
                },
                None => bail!("No member or user_id in reaction event"),
            },
        }
    }

    async fn handle_reaction_event(&self, ctx: Context, reaction: Reaction, msg: &ReactionEvent) {
        debug!("Handling reaction event");

        if reaction
            .guild_id
            .is_none_or(|id| id != self.base.config.reaction_logging.log_guild)
        {
            debug!("Ignoring reaction in {}", {
                if let Some(guild_id) = reaction.guild_id {
                    guild_id.to_string()
                } else {
                    "DM".to_string()
                }
            });
            return;
        }

        let guild_id = GuildId::new(self.base.config.reaction_logging.log_guild);
        let member = match self.get_member(&ctx, reaction.clone(), &guild_id).await {
            Ok(member) => member,
            Err(e) => {
                error!("Error getting member: {:?}", e);
                return;
            }
        };
        let channel_id = ChannelId::new(self.base.config.reaction_logging.log_channel);
        let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();
        let fmt = self.build_formatter(&member, &guild, &channel_id, &reaction.emoji);

        let color = if msg.color.random {
            Color::new(rand::random::<u32>() % 16777215)
        } else {
            Color::new(msg.color.color)
        };

        if let Err(e) = channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .title(fmt(&msg.title))
                        .description(fmt(&msg.content))
                        .footer(
                            CreateEmbedFooter::new(fmt(&msg.footer.text))
                                .icon_url(fmt(&msg.footer.icon)),
                        )
                        .color(color),
                ),
            )
            .await
        {
            error!("Error sending message: {:?}", e);
        }
    }
}

#[async_trait]
impl EventHandler for ReactionLogger {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        self.handle_reaction_event(ctx, reaction, &self.base.lang.reaction_logger.added)
            .await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        self.handle_reaction_event(ctx, reaction, &self.base.lang.reaction_logger.removed)
            .await;
    }
}
