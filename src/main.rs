use comics;
use anyhow::Result;

fn main() -> Result<()> {
    comics::run()?;
    Ok(())
}
