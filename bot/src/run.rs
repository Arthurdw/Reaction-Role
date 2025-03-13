use std::sync::Arc;

use anyhow::{Result, bail};

use crate::{
    config::BotConfig,
    features::{
        BaseHandler, info::InfoHandler, presence::PresenceHandler, reaction_logger::ReactionLogger,
        reaction_roles::ReactionRoles,
    },
};
use poise::serenity_prelude::{self as serenity, GatewayIntents};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

fn get_token(config: &BotConfig) -> Result<String> {
    if config.token.token_env_enabled {
        return match std::env::var(&config.token.token_env) {
            Ok(token) => Ok(token),
            Err(_) => bail!(
                "token_env is enabled but `{}` variable is not set",
                config.token.token_env
            ),
        };
    }

    Ok(config.token.token.clone())
}

pub async fn start(cfg: Arc<BotConfig>) -> Result<()> {
    let lang_config = Arc::new(crate::config::lang::load()?);
    let base_handler = Arc::new(BaseHandler::new(cfg.clone(), lang_config.clone()));

    let intents = GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(get_token(&cfg)?, intents)
        .framework(framework)
        .event_handler(ReactionRoles::new()?)
        .event_handler(InfoHandler);

    if cfg.reaction_logging.enabled {
        client = client.event_handler(ReactionLogger::new(base_handler.clone()));
    }

    if cfg.bot.rich_presence_enabled {
        client = client.event_handler(PresenceHandler::new(base_handler.clone()));
    }

    let _reaction_roles = Arc::new(crate::config::reaction_roles::load()?);

    Ok(client.await?.start().await?)
}
