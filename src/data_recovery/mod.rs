pub(crate) mod parse_existing_dir;

pub use parse_existing_dir::perform as parse_existing_dir;

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
