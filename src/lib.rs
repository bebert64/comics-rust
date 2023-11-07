#![allow(non_snake_case)]

pub mod data_recovery;
mod diesel_helpers;
pub mod rest;
mod schema;

use {
    don_error::DonResult,
    std::{env::var, path::PathBuf},
};

pub use diesel_helpers::db;

#[macro_use]
extern crate serde_derive;

fn comics_root_path<'l>(subdir: Option<&'l str>) -> DonResult<PathBuf> {
    let mut comics_root_path = PathBuf::from(var("COMICS_ROOT_PATH")?);
    if let Some(subdir) = subdir {
        comics_root_path.push(subdir);
    }
    Ok(comics_root_path)
}
