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

fn nas_path() -> ComicsResult<PathBuf> {
    dotenv()?;
    Ok(PathBuf::from(var("NAS_PATH")?))
}
