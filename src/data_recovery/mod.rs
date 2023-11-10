mod find_archives;
pub(crate) mod parse_existing_dir;
mod remove_archives;
mod remove_ea_dirs;
mod unzip;

pub use {
    find_archives::perform as find_archives, parse_existing_dir::perform as parse_existing_dir,
    remove_archives::perform as remove_archives, remove_ea_dirs::perform as remove_ea_dirs,
    unzip::perform as unzip,
};

pub use parse_existing_dir::{BookOrIssue, ParsingMode};

use crate::{comics_root_path, schema::archives};

use {diesel::prelude::*, diesel_derive_enum::DbEnum, don_error::DonResult, std::path::PathBuf};

#[derive(Queryable, Selectable, Insertable, Serialize)]
pub(crate) struct Archive {
    pub id: i32,
    pub path: String,
    pub status: ArchiveStatus,
}

#[derive(Debug, Clone, Copy, DbEnum, PartialEq, Eq, Hash, Serialize)]
pub(crate) enum ArchiveStatus {
    ToUnzip,
    ToParse,
    ToParseIssues,
    ToCompleteIssues,
    ToSearchComicVineId,
    Ok,
}

impl Archive {
    pub(crate) fn into_comics_dir(self: &Self) -> DonResult<PathBuf> {
        let comics_root = comics_root_path(Some("Comics"))?;
        Ok(comics_root.join({
            let mut subdir = self.path.clone();
            subdir.truncate(self.path.len() - 4);
            subdir
        }))
    }
}
