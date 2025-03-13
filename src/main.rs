use anyhow::Result;
use std::path::Path;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().ok();
    let bot_config = bot::config::load()?;
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(bot_config.console.tracing_level())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    debug!("Set tracing level to {}", bot_config.console.level);

    if Path::new(".env").exists() {
        debug!("Loading .env file...");
        dotenvy::dotenv()?;
        debug!("Loaded .env file");
    }

    info!("Starting bot...");
    bot::run(&bot_config).await
}
