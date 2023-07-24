mod find_archives;
mod parse_existing_dir;
mod remove_ea_dirs;
mod unzip;

pub use {
    find_archives::perform as find_archives, parse_existing_dir::perform as parse_existing_dir,
    remove_ea_dirs::perform as remove_ea_dirs, unzip::perform as unzip,
};

use crate::{nas_path, schema::archives, ComicsResult};

use {diesel::prelude::*, diesel_derive_enum::DbEnum, std::path::PathBuf};

#[derive(Queryable, Selectable, Insertable)]
struct Archive {
    pub id: i32,
    pub path: String,
    pub status: ArchiveStatus,
}

#[derive(Debug, Clone, Copy, DbEnum, PartialEq, Eq, Hash, Serialize)]
enum ArchiveStatus {
    Found,
    Unzipped,
    ParsedType,
    ParsedInfo,
    HasComicsVineId,
}

impl Archive {
    fn into_comics_dir(self: &Self) -> ComicsResult<PathBuf> {
        let comics_root = nas_path(Some("Comics"))?;
        Ok(comics_root.join({
            let mut subdir = self.path.clone();
            subdir.truncate(self.path.len() - 4);
            subdir
        }))
    }
}
