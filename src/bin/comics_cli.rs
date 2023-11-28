use {
    clap::{Parser, Subcommand},
    don_error::*,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[clap(long)]
        mode: String,
        #[clap(long)]
        dir_names: String,
    },
    Rename {
        #[clap(long)]
        from: String,
        #[clap(long)]
        to: String,
    },
    Test {
        #[clap(long)]
        dirs: String,
    },
}

fn main() -> DonResult<()> {
    let current_exe = std::env::current_exe()?;
    let current_dir = current_exe.parent().ok_or_don_err("Should have a parent")?;
    if dotenv::from_path("./.envrc").is_err() {
        dotenv::from_path(current_dir.join(".envrc_comics_cli")).map_err(|_| {
            err_msg!(
                "Couldn't find .envrc in execution dir or .envrc_comics_cli in {current_dir:?}"
            )
        })?;
    }
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Parse { mode, dir_names }) => comics::parse_existing_dir(mode, dir_names)?,
        Some(Commands::Rename { from, to }) => comics::rename(from, to)?,
        Some(Commands::Test { dirs }) => comics::test(dirs)?,
        None => (),
    }
    Ok(())
}
