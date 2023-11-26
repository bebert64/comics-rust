mod parsable;

use parsable::Parsable;
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
        collections::HashMap,
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
                    .filter(schema::archives::path.like(format!("{}.%", dir_relative_path)))
                    .get_only_result::<i32>(db)?;
                let book = parse_dir(&dir_path, mode)?;
                println!("Parsed {dir_name} (id: {archive_id}): {book:#?}");
                match book {
                    BookTypeOther::GraphicNovel(graphic_novel) => {
                        insert_into(schema::books::table)
                            .values((
                                schema::books::title.eq(graphic_novel.title),
                                schema::books::path
                                    .eq(graphic_novel.path.to_str().ok_or_don_err("TODO")?),
                            ))
                            .execute(db)?;
                    }
                    BookTypeOther::SingleVolume(book) => {
                        let volume_id = insert_into(schema::volumes::table)
                            .values(schema::volumes::name.eq(&book.volume))
                            .on_conflict(schema::volumes::name)
                            .do_update()
                            .set(schema::volumes::name.eq(&book.volume))
                            .returning(schema::volumes::id)
                            .get_only_result::<i32>(db)?;
                        let book_id = insert_into(schema::books::table)
                            .values((
                                schema::books::title.eq(book.title),
                                schema::books::volume_id.eq(volume_id),
                                schema::books::volume_number
                                    .eq(book.volume_number.map(|n| n as i32)),
                                schema::books::path.eq(book.path.to_str().ok_or_don_err("TODO")?),
                            ))
                            .returning(schema::books::id)
                            .get_only_result::<i32>(db)?;
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
                                        schema::issues::volume_id.eq(volume_id),
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
                                        schema::books__additional_files::position
                                            .eq(position as i32),
                                    ))
                                    .execute(db)?;
                                Ok(())
                            })?;
                    }
                };
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

pub(crate) fn parse_dir(directory: &Path, mode: ParsingMode) -> DonResult<BookTypeOther> {
    let FilesAndSubdirs { files, subdirs } = files_and_subdirs_cleaned_and_sorted(directory)?;
    match mode {
        ParsingMode::GraphicNovel => {
            if !subdirs.is_empty() || files.len() < 50 {
                bail!(format!(
                    "Parsing mode GraphicNovel should only be used \
                    on dirs with no subdir and more than 50 pages (dir name: {})",
                    file_name(directory)?,
                ))
            }
            let parsed_data = parsable::Title::parse(directory)?;
            Ok(BookTypeOther::GraphicNovel(GraphicNovel {
                title: parsed_data.title,
                path: directory.into(),
            }))
        }
        ParsingMode::SingleVolume => {
            let parsed_volume = parsable::Volume::parse(directory)?;
            Ok(BookTypeOther::SingleVolume(SingleVolume {
                issues_sorted: issues_from_subdirs(subdirs, &parsed_volume.volume, None)?,
                additional_files_sorted: files,
                volume: parsed_volume.volume,
                volume_number: parsed_volume.volume_number,
                title: None,
                path: directory.into(),
            }))
        }
        ParsingMode::SingleVolumeWithIssues => {
            if subdirs.is_empty() {
                bail!(format!(
                    "Parsing mode SingleVolumeWithIssues should only be used \
                    on dirs with sub-directories (dir name: {})",
                    file_name(directory)?,
                ))
            }
            let parsed_volume = parsable::VolumeWithIssues::parse(directory)?;
            Ok(BookTypeOther::SingleVolume(SingleVolume {
                volume: parsed_volume.volume.clone(),
                volume_number: parsed_volume.volume_number,
                title: None,
                issues_sorted: issues_from_subdirs(
                    subdirs,
                    &parsed_volume.volume,
                    Some((parsed_volume.first_issue, parsed_volume.last_issue)),
                )?,
                additional_files_sorted: files,
                path: directory.into(),
            }))
        }
        ParsingMode::SingleVolumeWithTitle => {
            let parsed_volume = parsable::VolumeWithTitle::parse(directory)?;
            Ok(BookTypeOther::SingleVolume(SingleVolume {
                issues_sorted: issues_from_subdirs(subdirs, &parsed_volume.volume, None)?,
                volume: parsed_volume.volume,
                volume_number: parsed_volume.volume_number,
                title: Some(parsed_volume.title),
                additional_files_sorted: files,
                path: directory.into(),
            }))
        }
    }
}

pub fn file_name(directory: &Path) -> DonResult<&str> {
    Ok(directory
        .file_name()
        .ok_or_don_err("directory should have a file_name")?
        .to_str()
        .ok_or_don_err("directory should have a valid OsStr name")?)
}

fn issues_from_subdirs(
    subdirs: Vec<PathBuf>,
    volume_from_parent: &str,
    first_and_last_issue: Option<(usize, usize)>,
) -> DonResult<Vec<Issue>> {
    let issues = subdirs
        .into_iter()
        .map(|subdir| {
            let parsed_issue = parsable::Issue::parse(&subdir)?;
            if parsed_issue
                .volume
                .as_ref()
                .is_some_and(|volume| volume != volume_from_parent)
            {
                bail!(format!(
                    "Issue {:#?} has a different volume than the one in the parent dir",
                    subdir
                ))
            }
            Ok(Issue {
                volume: parsed_issue.volume.unwrap_or(volume_from_parent.to_owned()),
                number: parsed_issue.number,
                path: Some(subdir),
            })
        })
        .collect::<DonResult<Vec<_>>>()?;
    if let Some((first_issue, last_issue)) = first_and_last_issue {
        let mut issues_by_number = issues
            .iter()
            .map(|issue| (issue.number, issue))
            .collect::<HashMap<_, _>>();
        (first_issue..last_issue + 1).try_for_each(|issue_number| -> DonResult<()> {
            issues_by_number
                .remove(&issue_number)
                .ok_or_don_err(format!("Issue {} not found", issue_number))?;
            Ok(())
        })?;
        if !issues_by_number.is_empty() {
            bail!(format!("Found extra issues: {:#?}", issues_by_number))
        }
    }
    Ok(issues)
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
