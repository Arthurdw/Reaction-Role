use anyhow::Result;
use std::path::Path;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().ok();
    tracing_subscriber::fmt::init();

    if Path::new(".env").exists() {
        debug!("Loading .env file...");
        dotenvy::dotenv()?;
        debug!("Loaded .env file");
    }

    info!("Starting bot...");
    bot::run().await
}
