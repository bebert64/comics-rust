#![allow(non_snake_case)]

pub mod actix;
mod comics_error;
pub mod data_recovery;
mod diesel_helpers;
mod schema;

pub use comics_error::ComicsResult;

use {
    dotenv::dotenv,
    std::{env::var, path::PathBuf},
};

#[macro_use]
extern crate serde_derive;

fn nas_path<'l>(subdir: Option<&'l str>) -> ComicsResult<PathBuf> {
    dotenv()?;
    let mut nas_path = PathBuf::from(var("NAS_PATH")?);
    if let Some(subdir) = subdir {
        nas_path.push(subdir);
    }
    Ok(nas_path)
}
