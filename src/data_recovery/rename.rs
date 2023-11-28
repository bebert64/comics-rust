use crate::{comics_root_path, schema};

use {
    diesel::{dsl::update, prelude::*},
    diesel_helpers::*,
    don_error::*,
    std::fs::rename,
};

pub fn perform(from: String, to: String) -> DonResult<()> {
    let from_cleaned = from[1..from.len() - 1].to_string().replace("'\\''", "'");

    let current_dir = std::env::current_dir()?;
    let comics_root = comics_root_path(None)?;
    let db = &mut db()?;
    let dir_path = current_dir.join(&from_cleaned);
    let dir_relative_path = dir_path
        .strip_prefix(&comics_root)
        .map_err(|err| err_msg!("{err}. dir_path: {dir_path:?}"))?
        .to_str()
        .ok_or_don_err("Path should be displayable as str")?;
    let new_path = current_dir.join(&to);
    let new_relative_path = new_path
        .strip_prefix(&comics_root)
        .map_err(|err| err_msg!("{err}. dir_path: {dir_path:?}"))?
        .to_str()
        .ok_or_don_err("Path should be displayable as str")?;
    if !(update(schema::archives::table.filter(schema::archives::path.eq(dir_relative_path)))
        .set(schema::archives::path.eq(new_relative_path))
        .execute(db)?
        == 1)
    {
        bail!("Couldn't update archive '{dir_relative_path}'")
    };
    rename(dir_path, new_path)?;

    Ok(())
}
