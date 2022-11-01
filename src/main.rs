use anyhow::Result;
use comics;

#[tokio::main]
async fn main() -> Result<()> {
    comics::run().await?;
    Ok(())
}
