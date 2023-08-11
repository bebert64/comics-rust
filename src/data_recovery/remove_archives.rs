use super::{Archive, ArchiveStatus};

use crate::{diesel_helpers::db, nas_path, schema, DonResult};

use {
    diesel::prelude::*,
    don_error::{bail, try_or_report},
    std::fs::File,
    walkdir::WalkDir,
};

pub fn perform() -> DonResult<()> {
    let mut db = db()?;
    let archives = schema::archives::table
        .select(Archive::as_select())
        .filter(schema::archives::status.ne(ArchiveStatus::ToUnzip))
        .get_results(&mut db)?;
    let comics_zipped_root = nas_path(Some("Comics_zipped"))?;
    println!("Clearing {} archives", archives.len());
    let total = archives.len();
    let mut counter = 0;
    for archive in archives.into_iter() {
        let archive_path = comics_zipped_root.join(&archive.path);
        if archive_path.exists() {
            try_or_report(|| {
                println!("Starting check for {}", &archive.path);
                let archive_file = File::open(&archive_path)?;
                let mut archive_zip = zip::ZipArchive::new(&archive_file)?;
                let outdir = archive.into_comics_dir()?;

                if !outdir.exists() {
                    diesel::update(schema::archives::table.find(archive.id))
                        .set(schema::archives::status.eq(ArchiveStatus::ToUnzip))
                        .execute(&mut db)?;
                    bail!("Matching unzipped dir does not exists");
                }

                let mut counter_file_archive = 0;
                for i in 0..archive_zip.len() {
                    let file = archive_zip.by_index(i)?;
                    if !(*file.name()).ends_with('/') {
                        counter_file_archive += 1;
                    }
                }

                let mut counter_file_dir = 0;
                let walk_dir = WalkDir::new(outdir).into_iter();
                for entry in walk_dir {
                    if entry?.file_type().is_file() {
                        counter_file_dir += 1;
                    }
                }

                if counter_file_archive == counter_file_dir {
                    println!("OK, found {counter_file_archive} in both, removing");
                    std::fs::remove_file(archive_path)?;
                } else {
                    println!("Number of files not matching : {counter_file_archive} in archive / {counter_file_dir} in dir");
                    diesel::update(schema::archives::table.find(archive.id))
                        .set(schema::archives::status.eq(ArchiveStatus::ToUnzip))
                        .execute(&mut db)?;
                }

                Ok(())
            })
        }
        counter += 1;
        if counter % 50 == 0 {
            println!("{counter}/{total} archives treated");
        }
    }
    Ok(())
}
