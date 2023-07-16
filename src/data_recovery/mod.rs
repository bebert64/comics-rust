mod find_archives;
mod parse_existing_dir;
mod remove_ea_dirs;
mod unzip_all;

pub use {
    find_archives::perform as find_archives, parse_existing_dir::perform as parse_existing_dir,
    remove_ea_dirs::perform as remove_ea_dirs, unzip_all::perform as unzip_all,
};

use diesel_derive_enum::DbEnum;

#[derive(Debug, Clone, Copy, DbEnum, PartialEq, Eq, Hash, Serialize)]
enum ArchiveStatus {
    Found,
    Unzipped,
    ParsedType,
    ParsedInfo,
    HasComicsVineId,
}
