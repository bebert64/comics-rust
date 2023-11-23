use {
    clap::{Parser, Subcommand},
    don_error::DonResult,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Parse {},
    Test {},
}

fn main() -> DonResult<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Parse {}) => {
            comics_rust::parse_existing_dir(&comics_rust::ParsingMode::GraphicNovel)?
        }
        Some(Commands::Test {}) => comics_rust::test().unwrap(),
        None => (),
    }
    Ok(())
}
