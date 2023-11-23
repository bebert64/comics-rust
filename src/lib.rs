#![allow(non_snake_case)]

pub(crate) mod data_recovery;
pub mod rest;
mod schema;

use {
    don_error::DonResult,
    std::{env::var, path::PathBuf},
};

pub use data_recovery::parse::{perform as parse_existing_dir, ParsingMode};

#[macro_use]
extern crate serde_derive;

fn comics_root_path(subdir: Option<&str>) -> DonResult<PathBuf> {
    let mut comics_root_path = PathBuf::from(var("COMICS_ROOT_PATH")?);
    if let Some(subdir) = subdir {
        comics_root_path.push(subdir);
    }
    Ok(comics_root_path)
}

pub fn test() -> DonResult<()> {
    let book = data_recovery::parse::parse_dir(
        &comics_root_path(Some("Fini/Crosswind 01-06"))?,
        &ParsingMode::SingleVolumeWithIssues,
    )?;
    println!("{book:#?}");
    Ok(())
}
