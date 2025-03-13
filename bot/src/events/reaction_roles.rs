use anyhow::Result;

use poise::serenity_prelude::{Context, EventHandler, Reaction, ReactionType, RoleId, async_trait};
use tracing::info;

use crate::config::reaction_roles::{ReactionRolesConfig, load};

pub struct ReactionRoles {
    cfg: ReactionRolesConfig,
}

enum ReactionRoleAction {
    Add,
    Remove,
}

impl ReactionRoles {
    pub fn new() -> Result<Self> {
        info!("Enabled reaction roles");
        Ok(Self { cfg: load()? })
    }

    async fn handle_reaction_event(
        &self,
        ctx: Context,
        reaction: Reaction,
        action: ReactionRoleAction,
    ) {
        let message = self.cfg.reaction_roles.get(&reaction.message_id.get());

        if message.is_none() {
            return;
        }
        let message = message.unwrap();

        let represented_emoij = match reaction.emoji {
            ReactionType::Custom { id, .. } => id.get().to_string(),
            ReactionType::Unicode(unicode) => unicode,
            _ => {
                info!("Unsupported reaction type");
                return;
            }
        };

        if let Some(role_id) = message
            .iter()
            .find(|(emoji, _)| &represented_emoij == emoji)
            .map(|(_, role_id)| role_id)
        {
            let guild_id = reaction.guild_id.unwrap();
            let member = guild_id
                .member(&ctx.http, reaction.user_id.unwrap())
                .await
                .unwrap();
            let role_id = RoleId::new(*role_id);
            let role = guild_id.role(&ctx.http, role_id).await.unwrap();

            info!(
                "User {} {} role {}",
                member.user.name,
                match action {
                    ReactionRoleAction::Add => "added",
                    ReactionRoleAction::Remove => "removed",
                },
                role.name
            );
            match action {
                ReactionRoleAction::Add => {
                    member.add_role(&ctx.http, role).await.unwrap();
                }
                ReactionRoleAction::Remove => {
                    member.remove_role(&ctx.http, role).await.unwrap();
                }
            }
        }
    }
}

#[async_trait]
impl EventHandler for ReactionRoles {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        self.handle_reaction_event(ctx, reaction, ReactionRoleAction::Add)
            .await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        self.handle_reaction_event(ctx, reaction, ReactionRoleAction::Remove)
            .await;
    }
}
