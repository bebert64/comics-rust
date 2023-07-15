use comics_rust::{scripts, ComicsResult};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Unzip {
        #[arg(short, long)]
        directory: String,
    },
    RemoveEaDirs {
        #[arg(short, long)]
        directory: String,
    },
    Parse {
        #[arg(short, long)]
        directory: String,
    },
}

fn main() -> ComicsResult<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Unzip { directory }) => scripts::unzip_all(&directory),
        Some(Commands::RemoveEaDirs { directory }) => scripts::remove_ea_dirs(&directory),
        Some(Commands::Parse { directory }) => scripts::parse_existing_dir(&directory),
        None => (),
    }
    Ok(())
}
