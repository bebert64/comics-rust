use super::{Archive, ArchiveStatus};

use crate::{
    comics_error::{err_msg, try_or_report},
    diesel_helpers::db,
    nas_path, schema, ComicsResult,
};

use {
    diesel::prelude::*,
    std::{
        fs::{read_dir, remove_dir, rename},
        path::{Path, PathBuf},
    },
};

pub fn perform() -> ComicsResult<()> {
    // let mut db = db()?;
    // let archives = schema::archives::table
    //     .select(Archive::as_select())
    //     .filter(schema::archives::status.eq(ArchiveStatus::ToParse))
    //     .get_results(&mut db)?;
    // let comics_root = nas_path(Some("Comics"))?;
    // for archive in archives.into_iter() {
    //     try_or_report(|| {
    //         let parsed_dir = parse_dir(&archive.into_comics_dir()?);
    //         Ok(())
    //     })
    // }
    Ok(())
}

// struct Issue {
//     volume: Volume,
//     number: usize,
//     dir: Option<PathBuf>,
// }

// struct Volume {
//     name: String,
// }

// struct Book {
//     issues: HashMap<usize, Issue>,
//     dir: Option<PathBuf>,
//     additional_files: Option<Vec<PathBuf>>,
// }

// struct ReadingOrder<T> {
//     elements: HashMap<usize, T>,
// }

#[derive(Debug)]
enum ParsedDir {
    Issue,
    BookWithNoIssue,
    BookWithIssues,
    BookWithIssuesAndBonus,
    IssueWithLowNumberOfFiles,
}

fn parse_dir(dir: &Path) -> ComicsResult<ParsedDir> {
    let (files, subdirs): (Vec<_>, Vec<_>) = read_dir(dir)?.into_iter().partition(|elem| {
        elem.as_ref()
            .expect("No reason it should fail ??")
            .path()
            .is_file()
    });

    fn remove_layers_in_subdirs(
        subdirs: Vec<Result<std::fs::DirEntry, std::io::Error>>,
    ) -> ComicsResult<()> {
        subdirs
            .into_iter()
            .try_for_each(|subdir| -> ComicsResult<()> {
                let subdir = subdir?;
                remove_extra_layers(&subdir.path())?;
                Ok(())
            })
    }

    Ok(match (subdirs.len(), files.len()) {
        (0, n) if n <= 10 => ParsedDir::IssueWithLowNumberOfFiles,
        (0, n) if n <= 50 => ParsedDir::Issue,
        (0, _) => ParsedDir::BookWithNoIssue,
        (1, 0) => {
            remove_extra_layers(dir)?;
            use ParsedDir::*;
            match parse_dir(dir)? {
                Issue => Issue,
                IssueWithLowNumberOfFiles => IssueWithLowNumberOfFiles,
                BookWithNoIssue => BookWithNoIssue,
                BookWithIssues | BookWithIssuesAndBonus => {
                    return err_msg(format!("Failed to parse {:?}", dir))
                }
            }
        }
        (1, _) => return err_msg(format!("Failed to parse {:?}", dir)),
        (_, 0) => {
            remove_layers_in_subdirs(subdirs)?;
            ParsedDir::BookWithIssues
        }
        (_, _) => {
            remove_layers_in_subdirs(subdirs)?;
            ParsedDir::BookWithIssuesAndBonus
        }
    })
}

fn remove_extra_layers(directory: &Path) -> ComicsResult<()> {
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
                        .expect("Should have a parent")
                        .to_path_buf()
                        .join(PathBuf::from(
                            old_file
                                .file_name()
                                .expect("Should have a name")
                                .to_str()
                                .expect("Should have a valid name"),
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
