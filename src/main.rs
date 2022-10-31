use comics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    comics::run()?;
    Ok(())
}
