use std::sync::Arc;

use anyhow::{Error, Result, bail};

use crate::{
    config::{BotConfig, lang::Lang},
    features::{
        BaseHandler,
        exp_system::{ExpSystemDatabase, ExpSystemMessageHandler, get_commands},
        info::InfoHandler,
        presence::PresenceHandler,
        reaction_logger::ReactionLogger,
        reaction_roles::ReactionRoles,
    },
};
use poise::{
    PrefixFrameworkOptions,
    serenity_prelude::{self as serenity, GatewayIntents},
};

pub struct Data {
    pub exp_db: Option<Arc<ExpSystemDatabase>>,
    pub config: Arc<BotConfig>,
    pub lang: Arc<Lang>,
}
pub type Context<'a> = poise::Context<'a, Data, Error>;

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
    let shared_config = cfg.clone();
    let exp_db = {
        if cfg.leveling.enabled {
            Some(Arc::new(ExpSystemDatabase::new().await?))
        } else {
            None
        }
    };

    let intents = GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_commands(),
            prefix_options: PrefixFrameworkOptions {
                prefix: if cfg.bot.prefix_enabled {
                    Some(cfg.bot.prefix.clone())
                } else {
                    None
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    exp_db,
                    config: shared_config,
                    lang: lang_config.clone(),
                })
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

    if cfg.leveling.enabled {
        client = client.event_handler(ExpSystemMessageHandler::new(
            base_handler.clone(),
            Arc::new(ExpSystemDatabase::new().await?),
        ));
    }

    let _reaction_roles = Arc::new(crate::config::reaction_roles::load()?);

    Ok(client.await?.start().await?)
}
