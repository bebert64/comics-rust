use super::ArchiveStatus;

use crate::{diesel_helpers::db, nas_path, schema, DonResult};

use {
    diesel::prelude::*,
    don_error::{try_or_report, DonResultOptionExtensions},
    walkdir::WalkDir,
};

pub fn perform(dir: &str) -> DonResult<()> {
    let comics_root = nas_path(Some("Comics_zipped"))?;
    let dir_path = comics_root.clone().join(dir);
    let walk_dir = WalkDir::new(dir_path).into_iter();
    for entry in walk_dir.filter_entry(|e| {
        !(e.file_name()
            .to_str()
            .is_some_and(|s| s == "14 Planet of the Apes issues"))
    }) {
        try_or_report(|| -> DonResult<_> {
            let entry = entry?;
            if entry.file_type().is_file() {
                if entry.file_name().to_str().is_some_and(|s| {
                    s.ends_with(".cbr")
                        || s.ends_with(".cbz")
                        || s.ends_with(".zip")
                        || s.ends_with(".rar")
                }) {
                    let relative_path = entry.path().strip_prefix(&comics_root)?;
                    diesel::insert_into(schema::archives::table)
                        .values((
                            schema::archives::path
                                .eq(relative_path.to_str().ok_or_don_err("Should have a path")?),
                            schema::archives::status.eq(ArchiveStatus::ToUnzip),
                        ))
                        .execute(&mut db()?)?;
                }
            }
            Ok(())
        });
    }
    Ok(())
}
