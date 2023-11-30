mod parsable;

pub use parsable::ParsingMode;

use super::structs::*;

use crate::{comics_root_path, schema};

use {
    diesel::{
        dsl::{insert_into, update},
        prelude::*,
    },
    diesel_helpers::*,
    don_error::*,
    std::{
        fs::{create_dir_all, read_dir, remove_dir, rename},
        path::{Path, PathBuf},
    },
};

pub fn perform(mode: String, dir_names: String) -> DonResult<()> {
    let mode = ParsingMode::from(&mode)?;
    let current_dir = std::env::current_dir()?;
    let comics_root = comics_root_path(None)?;
    let db = &mut db()?;
    dir_names[1..dir_names.len() - 1]
        .split("' '")
        .for_each(|dir_name| {
            try_or_report(|| {
                let dir_path = current_dir.join(&dir_name.replace("'\\''", "'"));
                let dir_relative_path = dir_path
                    .strip_prefix(&comics_root)
                    .map_err(|err| err_msg!("{err}. dir_path: {dir_path:?}"))?
                    .to_str()
                    .ok_or_don_err("Path should be displayable as str")?;
                let archive_id = schema::archives::table
                    .select(schema::archives::id)
                    .filter(schema::archives::path.eq(dir_relative_path))
                    .get_only_result::<i32>(db)?;
                let book = parse_dir(&dir_path, mode)?;
                println!("Parsed {dir_name} (id: {archive_id}): {book:#?}");
                let volume_id = book
                    .volume
                    .map(|volume| {
                        insert_into(schema::volumes::table)
                            .values(schema::volumes::name.eq(volume.clone()))
                            .on_conflict(schema::volumes::name)
                            .do_update()
                            .set(schema::volumes::name.eq(volume.clone()))
                            .returning(schema::volumes::id)
                            .get_only_result::<i32>(db)
                    })
                    .transpose()?;
                let book_id = insert_into(schema::books::table)
                    .values((
                        schema::books::title.eq(book.title),
                        schema::books::volume_id.eq(volume_id),
                        schema::books::volume_number.eq(book.volume_number.map(|n| n as i32)),
                        schema::books::path.eq(book.path.to_str().ok_or_don_err("TODO")?),
                    ))
                    .returning(schema::books::id)
                    .get_only_result::<i32>(db)?;
                book.position_in_reading_order
                    .map(
                        |PositionInReadingOrder {
                             position,
                             reading_order,
                         }|
                         -> DonResult<()> {
                            let reading_order_id = insert_into(schema::reading_orders::table)
                                .values(schema::reading_orders::name.eq(&reading_order))
                                .on_conflict(schema::reading_orders::name)
                                .do_update()
                                .set(schema::reading_orders::name.eq(reading_order.clone()))
                                .returning(schema::reading_orders::id)
                                .get_only_result::<i32>(db)?;
                            insert_into(schema::reading_orders__books::table)
                                .values((
                                    schema::reading_orders__books::book_id.eq(book_id),
                                    schema::reading_orders__books::reading_order_id
                                        .eq(reading_order_id),
                                    schema::reading_orders__books::position.eq(position as i32),
                                ))
                                .execute(db)?;
                            Ok(())
                        },
                    )
                    .transpose()?;
                book.issues_sorted.into_iter().enumerate().try_for_each(
                    |(position, issue)| -> DonResult<_> {
                        let issue_path = issue
                            .path
                            .map(|p: PathBuf| -> DonResult<_> {
                                Ok(p.to_str().ok_or_don_err("TODO")?.to_owned())
                            })
                            .transpose()?;
                        let issue_id = insert_into(schema::issues::table)
                            .values((
                                schema::issues::number.eq(issue.number as i32),
                                schema::issues::volume_id.eq(volume_id
                                    .ok_or_don_err("Can't have issues if there are no volume")?),
                                schema::issues::path.eq(issue_path),
                            ))
                            .returning(schema::issues::id)
                            .get_only_result::<i32>(db)?;
                        insert_into(schema::books__issues::table)
                            .values((
                                schema::books__issues::book_id.eq(book_id),
                                schema::books__issues::issue_id.eq(issue_id),
                                schema::books__issues::position.eq(position as i32),
                            ))
                            .execute(db)?;
                        Ok(())
                    },
                )?;
                book.additional_files_sorted
                    .into_iter()
                    .enumerate()
                    .try_for_each(|(position, file)| -> DonResult<_> {
                        let file_path = file.to_str().ok_or_don_err("TODO")?;
                        insert_into(schema::books__additional_files::table)
                            .values((
                                schema::books__additional_files::book_id.eq(book_id),
                                schema::books__additional_files::file_path.eq(file_path),
                                schema::books__additional_files::position.eq(position as i32),
                            ))
                            .execute(db)?;
                        Ok(())
                    })?;
                update(schema::archives::table.find(archive_id))
                    .set(schema::archives::status.eq(ArchiveStatus::ToSearchComicVineId))
                    .execute(db)?;
                let new_dir = comics_root
                    .parent()
                    .ok_or_don_err("TODO")?
                    .join("Comics OK")
                    .join(dir_relative_path);
                if !new_dir.exists() {
                    create_dir_all(&new_dir)?;
                }
                rename(dir_path, new_dir)?;
                Ok(())
            })
        });
    Ok(())
}

pub(crate) fn parse_dir(directory: &Path, mode: ParsingMode) -> DonResult<Book> {
    let FilesAndSubdirs { files, subdirs } = files_and_subdirs_cleaned_and_sorted(directory)?;
    if matches!(mode, ParsingMode::GraphicNovel) && (!subdirs.is_empty() || files.len() < 50) {
        bail!(format!(
            "Parsing mode GraphicNovel should only be used \
            on dirs with no subdir and more than 50 pages (dir name: {})",
            file_name(directory)?,
        ))
    }
    let parsed_book = parsable::parse_book(directory, mode)?;
    Ok(Book {
        additional_files_sorted: {
            if subdirs.is_empty() {
                Vec::new()
            } else {
                files
            }
        },
        issues_sorted: issues_from_subdirs(
            directory,
            subdirs,
            parsed_book.volume.as_deref(),
            parsed_book.issue_numbers,
        )?,
        volume: parsed_book.volume,
        volume_number: parsed_book.volume_number,
        title: parsed_book.title,
        position_in_reading_order: parsed_book.position_in_reading_order,
        path: directory.into(),
    })
}

pub fn file_name(directory: &Path) -> DonResult<&str> {
    Ok(directory
        .file_name()
        .ok_or_don_err("directory should have a file_name")?
        .to_str()
        .ok_or_don_err("directory should have a valid OsStr name")?)
}

fn issues_from_subdirs(
    parent_dir: &Path,
    subdirs: Vec<PathBuf>,
    volume_from_parent: Option<&str>,
    issue_numbers: Vec<usize>,
) -> DonResult<Vec<Issue>> {
    Ok(match (issue_numbers.len(), subdirs.len()) {
        (0, 0) => Vec::new(),
        (0, _) => bail!("This should work for multi volume but not yet implemented"),
        (1, 0) => vec![Issue {
            volume: volume_from_parent
                .ok_or_don_err(
                    "Impossible to create SingleIssue if \
            no volume_from_parent is provided",
                )?
                .to_owned(),
            number: *issue_numbers
                .get(0)
                .ok_or_don_err("Just checked non empty")?,
            path: Some(parent_dir.into()),
        }],
        (_, 0) => issue_numbers
            .into_iter()
            .map(|issue_number| {
                Ok(Issue {
                    volume: volume_from_parent
                        .ok_or_don_err(
                            "Impossible to create issues from only issue numbers if \
                        no volume_from_parent is provided",
                        )?
                        .to_owned(),
                    number: issue_number,
                    path: None,
                })
            })
            .collect::<DonResult<Vec<_>>>()?,
        (i, s) if i == s => subdirs
            .into_iter()
            .zip(issue_numbers.into_iter())
            .map(|(subdir, issue_number)| {
                if !file_name(&subdir)?.contains(&format!("{issue_number:02}")) {
                    bail!(format!(
                        "Issue number {issue_number} not found in subdir {subdir:?}"
                    ))
                }
                Ok(Issue {
                    volume: volume_from_parent
                        .ok_or_don_err(
                            "Impossible to create issues for a SingleVolume if \
                            no volume_from_parent is provided",
                        )?
                        .to_owned(),
                    number: issue_number,
                    path: Some(subdir),
                })
            })
            .collect::<DonResult<Vec<_>>>()?,
        (i, s) => bail!("{i} issues and {s} subdirs found, should be equal"),
    })
}

fn files_and_subdirs_cleaned_and_sorted(dir: &Path) -> DonResult<FilesAndSubdirs> {
    remove_extra_layers(dir)?;
    let files_and_subdirs = files_and_subdirs(dir)?;
    files_and_subdirs
        .subdirs
        .iter()
        .try_for_each(|subdir| -> DonResult<()> {
            remove_extra_layers(&subdir)?;
            Ok(())
        })?;
    Ok(files_and_subdirs)
}
fn files_and_subdirs(dir: &Path) -> DonResult<FilesAndSubdirs> {
    let (mut files, mut subdirs) = read_dir(dir)?
        .map(|result| -> DonResult<_> { Ok(result?.path()) })
        .collect::<DonResult<Vec<_>>>()?
        .into_iter()
        .partition::<Vec<_>, _>(|path| path.is_file());
    files.sort_unstable();
    subdirs.sort_unstable();
    Ok(FilesAndSubdirs { files, subdirs })
}
// If directory contains only one subdir, move all files from this subdir to the parent
// directory remove the subdir and repeat until there's either no subdir or more than one
// subdir
fn remove_extra_layers(directory: &Path) -> DonResult<()> {
    let mut loop_ctrl = true;
    while loop_ctrl {
        loop_ctrl = false;
        let FilesAndSubdirs { files, mut subdirs } = files_and_subdirs(directory)?;
        match (subdirs.len(), files.len()) {
            (1, 0) => {
                let dir_to_remove = subdirs.pop().ok_or_don_err("Just checked not empty")?;
                for file in read_dir(&dir_to_remove)? {
                    let file = file?;
                    let old_file = file.path();
                    let new_file = &dir_to_remove
                        .parent()
                        .ok_or_don_err("Should have a parent")?
                        .to_path_buf()
                        .join(PathBuf::from(
                            old_file
                                .file_name()
                                .ok_or_don_err("Should have a name")?
                                .to_str()
                                .ok_or_don_err("Should have a valid name")?,
                        ));
                    println!("Moving {:#?} to {:#?}", &old_file, &new_file);
                    rename(old_file, new_file)?;
                }
                remove_dir(&dir_to_remove)?;
                loop_ctrl = true;
            }
            (1, _) => bail!("Should never have a single subdir with files"),
            _ => (),
        }
    }
    Ok(())
}
