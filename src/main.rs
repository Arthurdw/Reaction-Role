use anyhow::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    if Path::new(".env").exists() {
        dotenvy::dotenv()?;
    }

    bot::run().await
}
