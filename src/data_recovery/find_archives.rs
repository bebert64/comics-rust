use super::ArchiveStatus;

use crate::{comics_error::try_or_report, diesel_helpers::db, nas_path, schema, ComicsResult};

use {diesel::prelude::*, walkdir::WalkDir};

pub fn perform(dir: &str) -> ComicsResult<()> {
    let mut comics_root = nas_path()?;
    comics_root.push("Comics_unzipped");
    if !comics_root.exists() {
        panic!("not found")
    }
    let mut dir_path = comics_root.clone()?;
    dir_path.push(dir);
    let walk_dir = WalkDir::new(dir_path).into_iter();
    for entry in walk_dir.filter_entry(|e| {
        !(e.file_type().is_dir()
            && e.file_name()
                .to_str()
                .is_some_and(|s| s == "14 Planet of the Apes issues"))
    }) {
        try_or_report(|| {
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
                                .eq(relative_path.to_str().expect("Should have a path")),
                            schema::archives::status.eq(ArchiveStatus::Found),
                        ))
                        .execute(&mut db()?)?;
                }
            }
            Ok(())
        });
    }
    Ok(())
}
