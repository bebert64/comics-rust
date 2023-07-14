use comics_rust::scripts;

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
    Remove {
        #[arg(short, long)]
        directory: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Unzip { directory }) => scripts::unzip_all(&directory)?,
        Some(Commands::Remove { directory }) => scripts::remove_additional_directories(&directory)?,
        None => (),
    }
    Ok(())
}
