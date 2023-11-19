use super::{Archive, ArchiveStatus};

use crate::{comics_root_path, schema};

use {
    diesel::prelude::*,
    diesel_helpers::db,
    don_error::{try_or_report, DonResult, DonResultOptionExtensions},
    std::{
        fs::{create_dir_all, remove_dir_all, File},
        io::copy,
    },
};

pub fn perform() -> DonResult<()> {
    let mut db = db()?;
    let archives = schema::archives::table
        .select(Archive::as_select())
        .filter(schema::archives::status.eq(ArchiveStatus::ToUnzip))
        .get_results(&mut db)?;
    let comics_zipped_root = comics_root_path(Some("Comics_zipped"))?;
    println!("Unzipping {} archives", archives.len());
    for archive in archives.into_iter() {
        try_or_report(|| {
            println!("Starting extraction of {}", &archive.path);
            let archive_path = comics_zipped_root.join(&archive.path);
            let archive_file = File::open(&archive_path)?;
            let mut archive_zip = zip::ZipArchive::new(&archive_file)?;
            let outdir = archive.into_comics_dir()?;

            if outdir.exists() {
                remove_dir_all(&outdir)?;
            }

            let mut counter_file = 0;
            for i in 0..archive_zip.len() {
                let mut file = archive_zip.by_index(i)?;
                let outpath = outdir.join(
                    file.enclosed_name()
                        .ok_or_don_err("Unvalid file inside archive")?,
                );
                if (*file.name()).ends_with('/') {
                    create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            create_dir_all(p)?;
                        }
                    }
                    let mut outfile = File::create(&outpath)?;
                    copy(&mut file, &mut outfile)?;
                }
                counter_file += 1;
            }
            println!("Extracted {counter_file} files to {outdir:?}");
            diesel::update(schema::archives::table.find(&archive.id))
                .set(schema::archives::status.eq(ArchiveStatus::ToParse))
                .execute(&mut db)?;
            Ok(())
        })
    }
    Ok(())
}
