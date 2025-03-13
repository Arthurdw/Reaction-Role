use anyhow::{Result, bail};

use crate::config::BotConfig;
use crate::events::reaction_logger::ReactionLogger;
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

pub async fn start(cfg: &BotConfig) -> Result<()> {
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

    let client = serenity::ClientBuilder::new(get_token(cfg)?, intents)
        .event_handler(ReactionLogger)
        .framework(framework)
        .await;

    Ok(client.unwrap().start().await?)
}
