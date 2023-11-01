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

fn nas_path<'l>(subdir: Option<&'l str>) -> DonResult<PathBuf> {
    let mut nas_path = PathBuf::from(var("NAS_PATH")?);
    if let Some(subdir) = subdir {
        nas_path.push(subdir);
    }
    Ok(nas_path)
}
