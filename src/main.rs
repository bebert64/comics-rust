use comics_rust::{data_recovery, ComicsResult};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    RemoveEaDirs {
        #[arg(short, long)]
        directory: String,
    },
    Find {
        #[arg(short, long)]
        directory: String,
    },
    Unzip {
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
        Some(Commands::RemoveEaDirs { directory }) => data_recovery::remove_ea_dirs(&directory),
        Some(Commands::Find { directory }) => data_recovery::find_archives(&directory)?,
        Some(Commands::Unzip { directory }) => data_recovery::unzip_all(&directory),
        Some(Commands::Parse { directory }) => data_recovery::parse_existing_dir(&directory),
        None => (),
    }
    Ok(())
}
