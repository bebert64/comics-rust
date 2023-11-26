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
                        book.issues_sorted
                            .map(|issues| -> DonResult<_> {
                                Ok(issues.into_iter().enumerate().try_for_each(
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
                                )?)
                            })
                            .transpose()?;
                        book.additional_files_sorted
                            .map(|files| -> DonResult<_> {
                                Ok(files.into_iter().enumerate().try_for_each(
                                    |(position, file)| -> DonResult<_> {
                                        let file_path = file.to_str().ok_or_don_err("TODO")?;
                                        insert_into(schema::books__additional_files::table)
                                            .values((
                                                schema::books__additional_files::book_id
                                                    .eq(book_id),
                                                schema::books__additional_files::file_path
                                                    .eq(file_path),
                                                schema::books__additional_files::position
                                                    .eq(position as i32),
                                            ))
                                            .execute(db)?;
                                        Ok(())
                                    },
                                )?)
                            })
                            .transpose()?;
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
    match mode {
        ParsingMode::GraphicNovel => match directory_type(directory)? {
            DirectoryType::BookWithNoIssue => {
                let parsed_data = parsable::Title::parse(directory)?;
                Ok(BookTypeOther::GraphicNovel(GraphicNovel {
                    title: parsed_data.title,
                    path: directory.into(),
                }))
            }
            DirectoryType::Issue
            | DirectoryType::BookWithIssues { .. }
            | DirectoryType::BookWithIssuesAndBonus { .. } => {
                bail!(format!(
                    "Parsing mode GraphicNovel should only be used \
                    on dirs with no subdir and more than 50 pages (dir name: {})",
                    file_name(directory)?,
                ))
            }
        },
        ParsingMode::SingleVolume => match directory_type(directory)? {
            DirectoryType::BookWithNoIssue => {
                let parsed_volume = parsable::Volume::parse(directory)?;
                Ok(BookTypeOther::SingleVolume(SingleVolume {
                    volume: parsed_volume.volume,
                    volume_number: parsed_volume.volume_number,
                    title: None,
                    issues_sorted: None,
                    additional_files_sorted: None,
                    path: directory.into(),
                }))
            }
            DirectoryType::BookWithIssues { mut issues } => {
                let parsed_volume = parsable::Volume::parse(directory)?;
                Ok(BookTypeOther::SingleVolume(SingleVolume {
                    issues_sorted: {
                        issues.sort_unstable();
                        Some(
                            issues
                                .into_iter()
                                .map(|issue_dir| {
                                    let parsed_issue = parsable::Issue::parse(&issue_dir)?;
                                    if parsed_issue.volume.is_some_and(|volume| {
                                        volume != parsed_volume.volume
                                    }) {
                                        bail!(format!(
                                            "Issue {:#?} has a different volume than the one in the parent dir",
                                            issue_dir
                                        ))
                                    }
                                    Ok(Issue {
                                        volume: parsed_volume.volume.clone(),
                                        number: parsed_issue.number,
                                        path: Some(issue_dir),
                                    })
                                })
                                .collect::<DonResult<Vec<_>>>()?,
                        )
                    },
                    volume: parsed_volume.volume,
                    volume_number: parsed_volume.volume_number,
                    title: None,
                    additional_files_sorted: None,
                    path: directory.into(),
                }))
            }
            DirectoryType::BookWithIssuesAndBonus { .. } => {
                unimplemented!("TODO")
            }
            DirectoryType::Issue => {
                bail!(format!(
                    "Parsing mode SingleVolume cannot be used \
                    on dirs with no subdir and less than 50 pages (dir name: {})",
                    file_name(directory)?,
                ))
            }
        },
        ParsingMode::SingleVolumeWithIssues => match directory_type(directory)? {
            DirectoryType::BookWithIssues { issues } => {
                let parsed_volume = parsable::VolumeWithIssues::parse(directory)?;
                Ok(BookTypeOther::SingleVolume(SingleVolume {
                    volume: parsed_volume.volume.clone(),
                    volume_number: parsed_volume.volume_number,
                    title: None,
                    issues_sorted: Some(issues_sorted(issues, parsed_volume)?),
                    additional_files_sorted: None,
                    path: directory.into(),
                }))
            }
            DirectoryType::BookWithIssuesAndBonus {
                issues,
                mut additional_files,
            } => {
                let parsed_volume = parsable::VolumeWithIssues::parse(directory)?;
                Ok(BookTypeOther::SingleVolume(SingleVolume {
                    volume: parsed_volume.volume.clone(),
                    volume_number: parsed_volume.volume_number,
                    title: None,
                    issues_sorted: Some(issues_sorted(issues, parsed_volume)?),
                    additional_files_sorted: {
                        additional_files.sort_unstable();
                        Some(additional_files)
                    },
                    path: directory.into(),
                }))
            }
            _ => bail!(format!(
                "Parsing mode SingleVolumeWithIssues should only be used \
                on dirs with sub-directories (dir name: {})",
                file_name(directory)?,
            )),
        },
        ParsingMode::SingleVolumeWithTitle => match directory_type(directory)? {
            DirectoryType::BookWithNoIssue => {
                let parsed_volume = parsable::VolumeWithTitle::parse(directory)?;
                Ok(BookTypeOther::SingleVolume(SingleVolume {
                    volume: parsed_volume.volume,
                    volume_number: parsed_volume.volume_number,
                    title: Some(parsed_volume.title),
                    issues_sorted: None,
                    additional_files_sorted: None,
                    path: directory.into(),
                }))
            }
            DirectoryType::BookWithIssues { mut issues } => {
                let parsed_volume = parsable::VolumeWithTitle::parse(directory)?;
                Ok(BookTypeOther::SingleVolume(SingleVolume {
                    issues_sorted: {
                        issues.sort_unstable();
                        Some(
                            issues
                                .into_iter()
                                .map(|issue_dir| {
                                    let parsed_issue = parsable::Issue::parse(&issue_dir)?;
                                    if parsed_issue.volume.is_some_and(|volume| {
                                        volume != parsed_volume.volume
                                    }) {
                                        bail!(format!(
                                            "Issue {:#?} has a different volume than the one in the parent dir",
                                            issue_dir
                                        ))
                                    }
                                    Ok(Issue {
                                        volume: parsed_volume.volume.clone(),
                                        number: parsed_issue.number,
                                        path: Some(issue_dir),
                                    })
                                })
                                .collect::<DonResult<Vec<_>>>()?,
                        )
                    },
                    volume: parsed_volume.volume,
                    volume_number: parsed_volume.volume_number,
                    title: Some(parsed_volume.title),
                    additional_files_sorted: None,
                    path: directory.into(),
                }))
            }
            DirectoryType::BookWithIssuesAndBonus {
                mut issues,
                mut additional_files,
            } => {
                let parsed_volume = parsable::VolumeWithIssues::parse(directory)?;
                Ok(BookTypeOther::SingleVolume(SingleVolume {
                    issues_sorted: {
                        issues.sort_unstable();
                        Some(
                            issues
                                .into_iter()
                                .map(|issue_dir| {
                                    let parsed_issue = parsable::Issue::parse(&issue_dir)?;
                                    if parsed_issue.volume.is_some_and(|volume| {
                                        volume != parsed_volume.volume
                                    }) {
                                        bail!(format!(
                                            "Issue {:#?} has a different volume than the one in the parent dir",
                                            issue_dir
                                        ))
                                    }
                                    Ok(Issue {
                                        volume: parsed_volume.volume.clone(),
                                        number: parsed_issue.number,
                                        path: Some(issue_dir),
                                    })
                                })
                                .collect::<DonResult<Vec<_>>>()?,
                        )
                    },
                    volume: parsed_volume.volume,
                    volume_number: parsed_volume.volume_number,
                    title: None,
                    additional_files_sorted: {
                        additional_files.sort_unstable();
                        Some(additional_files)
                    },
                    path: directory.into(),
                }))
            }
            DirectoryType::Issue => {
                bail!(format!(
                    "Parsing mode SingleVolumeWithTitle cannot be used \
                    on dirs with no subdir and less than 50 pages (dir name: {})",
                    file_name(directory)?,
                ))
            }
        },
    }
}

fn issues_sorted(
    issue_dirs: Vec<PathBuf>,
    parsed_volume: parsable::VolumeWithIssues,
) -> DonResult<Vec<Issue>> {
    let mut issue_dirs_by_issue_number = issue_dirs
        .into_iter()
        .map(|issue_dir| {
            let parsed_issue = parsable::Issue::parse(&issue_dir)?;
            if parsed_issue
                .volume
                .is_some_and(|volume| volume != parsed_volume.volume)
            {
                bail!(format!(
                    "Issue {:#?} has a different volume than the one in the parent dir",
                    issue_dir
                ))
            }
            Ok((parsed_issue.number, issue_dir))
        })
        .collect::<DonResult<HashMap<_, _>>>()?;
    let issues_sorted = {
        let mut issues = (parsed_volume.first_issue..parsed_volume.last_issue + 1)
            .map(|issue_index| {
                let issue_dir = issue_dirs_by_issue_number
                    .remove(&issue_index)
                    .ok_or_don_err(format!("Issue {} not found", issue_index))?;
                Ok(Issue {
                    volume: parsed_volume.volume.clone(),
                    number: issue_index,
                    path: Some(issue_dir),
                })
            })
            .collect::<DonResult<Vec<_>>>()?;
        issues.sort_unstable_by_key(|issue| issue.number);
        if !issue_dirs_by_issue_number.is_empty() {
            bail!(format!(
                "Found extra issues: {:#?}",
                issue_dirs_by_issue_number
            ))
        }
        issues
    };
    Ok(issues_sorted)
}

fn directory_type(dir: &Path) -> DonResult<DirectoryType> {
    use DirectoryType::*;
    let (files, subdirs): (Vec<_>, Vec<_>) = read_dir(dir)?
        .map(|result| -> DonResult<_> { Ok(result?) })
        .collect::<DonResult<Vec<_>>>()?
        .into_iter()
        .partition(|elem| elem.path().is_file());

    fn remove_layers_in_subdirs(subdirs: &Vec<std::fs::DirEntry>) -> DonResult<()> {
        subdirs.into_iter().try_for_each(|subdir| -> DonResult<()> {
            remove_extra_layers(&subdir.path())?;
            Ok(())
        })
    }

    Ok(match (subdirs.len(), files.len()) {
        (0, n) if n <= 50 => Issue,
        (0, _) => BookWithNoIssue,
        (1, 0) => {
            remove_extra_layers(dir)?;
            use DirectoryType::*;
            match directory_type(dir)? {
                Issue => Issue,
                BookWithNoIssue => BookWithNoIssue,
                // If there's only one subdir, it should itself contains only images
                BookWithIssues { .. } | BookWithIssuesAndBonus { .. } => {
                    bail!(format!("Failed to parse {dir:?}"))
                }
            }
        }
        (1, _) => bail!(format!("Failed to parse {dir:?}")),
        (_, 0) => {
            remove_layers_in_subdirs(&subdirs)?;
            BookWithIssues {
                issues: subdirs.into_iter().map(|subdir| subdir.path()).collect(),
            }
        }
        (_, _) => {
            remove_layers_in_subdirs(&subdirs)?;
            BookWithIssuesAndBonus {
                issues: subdirs.into_iter().map(|subdir| subdir.path()).collect(),
                additional_files: files.into_iter().map(|file| file.path()).collect(),
            }
        }
    })
}

fn remove_extra_layers(directory: &Path) -> DonResult<()> {
    let mut loop_ctrl = true;
    while loop_ctrl {
        loop_ctrl = false;
        let mut files_in_entry = read_dir(directory)?.peekable();
        if let Some(first_file) = files_in_entry.next() {
            let first_file = first_file?.path();
            if first_file.is_dir() && files_in_entry.next().is_none() {
                let dir_to_remove = first_file;
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
        }
    }
    Ok(())
}

pub fn file_name(directory: &Path) -> DonResult<&str> {
    Ok(directory
        .file_name()
        .ok_or_don_err("directory should have a file_name")?
        .to_str()
        .ok_or_don_err("directory should have a valid OsStr name")?)
}
