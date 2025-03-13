mod config;
mod events;
mod run;
mod utils;

use anyhow::Result;

pub async fn run() -> Result<()> {
    let bot_config = config::config::load()?;

    run::start(&bot_config).await
}
